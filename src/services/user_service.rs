use chrono::naive::NaiveDateTime;
use chrono::NaiveDate;
use env_logger::builder;
use jsonwebtoken::{encode, Header};
use log::{debug, error, info, Level};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ActiveValue::Set, Database, DatabaseConnection, DbErr,
    EntityTrait, QuerySelect,
};
use sea_orm_migration::prelude::SimpleExpr;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Local, Utc};
use tonic::metadata::MetadataMap;
use tonic::{async_trait, Request, Response, Status};

use crate::entities::user as userEntity;
use crate::entities::user;
use crate::entities::{refresh_token, regular_token};
use crate::generated::user as genuser;
use crate::generated::user::{
    DeleteUserRequest, GetUserRequest, Response as UResponse, SignInRequest, SignOutRequest,
    UpdateUserRequest,
};
use crate::services::jwt_service;
use crate::services::jwt_service::JWTService;
use crate::userProto::user_service_server::UserService;
use crate::userProto::SignInResponse;
use crate::userProto::{CreateUserRequest, UserResponse};

#[derive(Debug, Default)]
pub struct MyUserService {
    pub(crate) db: DatabaseConnection,
    pub(crate) jwt_service: jwt_service::JWTService,
}

#[async_trait]
impl UserService for MyUserService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UResponse>, Status> {
        self.create_user(request).await
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        self.update_user(request).await
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        self.delete_user(request).await
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        self.get_user(request).await
    }

    async fn sign_in(
        &self,
        request: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        self.sign_in(request).await
    }

    async fn sign_out(
        &self,
        _request: Request<SignOutRequest>,
    ) -> Result<Response<UResponse>, Status> {
        self.sign_out(_request.metadata().clone(), _request).await
    }
}

impl MyUserService {
    pub fn new_user_service(
        db: DatabaseConnection,
        jwt_service: jwt_service::JWTService,
    ) -> MyUserService {
        MyUserService { db, jwt_service }
    }

    async fn get_user_service_db_connection(&self) -> Result<DatabaseConnection, Status> {
        Ok(self.db.clone())
    }
}

impl MyUserService {
    async fn sign_out(
        &self,
        metadata: MetadataMap,
        _request: Request<SignOutRequest>,
    ) -> Result<Response<UResponse>, Status> {
        let db = self.get_user_service_db_connection().await?;
        let token_value = if let Some(header_value) = metadata.get("authorization") {
            match header_value.to_str() {
                Ok(v) => v.to_string(),
                Err(_) => return Err(Status::invalid_argument("Invalid token format")),
            }
        } else {
            return Err(Status::invalid_argument("Token not found"));
        };
        let result = regular_token::Entity::find()
            .filter(regular_token::Column::Token.eq(token_value))
            .filter(regular_token::Column::Active.eq(true))
            .one(&db)
            .await;

        match result {
            Ok(Some(token)) => {
                let update_result = regular_token::Entity::update_many()
                    .filter(regular_token::Column::Token.eq(token.token.clone()))
                    .col_expr(regular_token::Column::Active, SimpleExpr::from(false))
                    .col_expr(regular_token::Column::Expired, SimpleExpr::from(true))
                    .col_expr(
                        regular_token::Column::ExpirationTime,
                        SimpleExpr::from(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
                    )
                    .exec(&db)
                    .await;

                match update_result {
                    Ok(_) => Ok(Response::new(UResponse {
                        status: true,
                        message: "Successfully signed out".to_string(),
                    })),
                    Err(e) => {
                        error!("Error: {:?}", e);
                        Err(Status::internal(format!("Failed to sign out: {}", e)))
                    }
                }
            }
            Ok(None) => Err(Status::not_found("Token not found or is already expired")),
            Err(e) => {
                error!("Error: {:?}", e);
                Err(Status::internal(format!("Failed to sign out: {}", e)))
            }
        }
    }

    async fn sign_in(
        &self,
        request: Request<SignInRequest>,
    ) -> Result<Response<SignInResponse>, Status> {
        let req = request.into_inner();
        if req.password.is_empty() || req.login_or_email.is_empty() {
            return Err(Status::internal("Fields cannot be empty"));
        }
        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(e) => {
                error!("Error: {:?}", e);
                return Err(Status::internal("Failed to get db connection"));
            }
        };

        let user_result = userEntity::Entity::find()
            .filter(
                user::Column::Username
                    .eq(req.login_or_email.clone())
                    .or(user::Column::Email.eq(req.login_or_email.clone())),
            )
            .filter(user::Column::Password.eq(req.password.clone()))
            .one(&db_conn)
            .await;

        match user_result {
            Err(e) => {
                error!("Error: {:?}", e);
                return Err(Status::internal(format!("Failed to sign in: {}", e)));
            }
            Ok(None) => return Err(Status::not_found("User not found")),
            Ok(Some(user)) => {
                let token_result = regular_token::Entity::find()
                    .filter(regular_token::Column::UserId.eq(user.id.clone()))
                    .filter(regular_token::Column::Active.eq(true))
                    .one(&db_conn)
                    .await;

                match token_result {
                    Err(_e) => {
                        let new_jwt = jwt_service::generate_jwt(user.clone()).await;
                        let new_token = regular_token::ActiveModel {
                            id: ActiveValue::NotSet,
                            user_id: Set(user.id.clone()),
                            token: Set(new_jwt.clone()),
                            active: Set(true),
                            creation_time: Set(NaiveDateTime::from_timestamp(
                                Utc::now().timestamp(),
                                0,
                            )),
                            expired: Set(false),
                            expiration_time: ActiveValue::NotSet,
                        };
                        let _token_insert_result = regular_token::Entity::insert(new_token.clone())
                            .exec(&db_conn)
                            .await
                            .map_err(|e| {
                                error!("Error: {:?}", e);
                                Status::internal(format!("Failed to Insert JWTtoken: {}", e))
                            })?;

                        Ok(Response::new(SignInResponse {
                            status: true,
                            message: "Successfully signed in :)".to_string(),
                            token: new_token.token.unwrap(),
                        }))
                    }
                    Ok(None) => {
                        let new_jwt = jwt_service::generate_jwt(user.clone()).await;
                        let new_token = regular_token::ActiveModel {
                            id: ActiveValue::NotSet,
                            user_id: Set(user.id.clone()),
                            token: Set(new_jwt.clone()),
                            active: Set(true),
                            creation_time: Set(NaiveDateTime::from_timestamp(
                                Utc::now().timestamp(),
                                0,
                            )),
                            expired: Set(false),
                            expiration_time: ActiveValue::NotSet,
                        };
                        let _token_insert_result = regular_token::Entity::insert(new_token.clone())
                            .exec(&db_conn)
                            .await
                            .map_err(|e| {
                                error!("Error: {:?}", e);
                                Status::internal(format!("Failed to Insert JWTtoken: {}", e))
                            })?;

                        Ok(Response::new(SignInResponse {
                            status: true,
                            message: "Successfully signed in :)".to_string(),
                            token: new_token.token.unwrap(),
                        }))
                    }
                    Ok(Some(token)) => {
                        return Ok(Response::new(SignInResponse {
                            status: true,
                            message: "Successfully signed in :)".to_string(),
                            token: token.token,
                        }));
                    }
                }
            }
        }
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UResponse>, Status> {
        let req = request.into_inner();

        if req.username.is_empty()
            || req.password.is_empty()
            || req.email.is_empty()
            || req.gender < 0
            || req.gender > 2
        {
            return Err(Status::internal("Fields cannot be empty"));
        }

        let user_exists = self.user_by_username_exists(req.username.clone()).await?;
        match user_exists {
            true => return Err(Status::internal("User already exists")),
            false => (),
        }
        let new_user = userEntity::ActiveModel {
            id: ActiveValue::NotSet,
            username: Set(req.username.clone()),
            password: Set(req.password.clone()),
            email: Set(req.email.clone()),
            gender: Set(req.gender.into()),
        };

        let insert_result = userEntity::Entity::insert(new_user.clone())
            .exec(&self.db)
            .await;
        match insert_result {
            Ok(_) => {
                info!("User created");
                Ok(Response::new(UResponse {
                    status: true,
                    message: "".to_string(),
                }))
            }
            Err(e) => {
                error!("Error: {:?}", e);
                if let DbErr::Conn(_) = e {
                    let db_conn = self.get_user_service_db_connection().await?;
                    let insert_result = userEntity::Entity::insert(new_user.clone())
                        .exec(&db_conn)
                        .await;
                    match insert_result {
                        Ok(_) => Ok(Response::new(UResponse {
                            status: true,
                            message: "Successfully Registered".to_string(),
                        })),
                        Err(e) => {
                            error!("Error: {:?}", e);
                            Err(Status::internal(format!("Failed to create user: {}", e)))
                        }
                    }
                } else {
                    Err(Status::internal(format!("Failed to create user: {}", e)))
                }
            }
        }
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        if req.username.is_empty()
            || req.password.is_empty()
            || req.email.is_empty()
            || req.gender < 0
            || req.gender > 2
        {
            error!("Fields cannot be empty");
            return Err(Status::invalid_argument("Fields cannot be empty"));
        }

        let user_check = self
            .get_user(Request::new(GetUserRequest { id: req.id }))
            .await;
        if user_check.is_err() {
            error!("User not found");
            return Err(Status::not_found("User not found"));
        }

        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(_) => self.get_user_service_db_connection().await?,
        };

        let updated_user = userEntity::ActiveModel {
            id: ActiveValue::Set(req.id),
            username: Set(req.username.clone()),
            password: Set(req.password.clone()),
            email: Set(req.email.clone()),
            gender: Set(req.gender.into()),
        };

        let update_result = userEntity::Entity::update(updated_user.clone())
            .exec(&db_conn)
            .await;
        match update_result {
            Ok(_) => {
                info!("User updated");
                Ok(Response::new(UserResponse {
                    id: req.id,
                    username: req.username.clone(),
                    email: req.email.clone(),
                    status: 1,
                }))
            }
            Err(e) => {
                error!("Error: {:?}", e);
                if let DbErr::Conn(_) = e {
                    let db_conn = self.get_user_service_db_connection().await?;
                    let update_result = userEntity::Entity::update(updated_user.clone())
                        .exec(&db_conn)
                        .await;
                    match update_result {
                        Ok(_) => Ok(Response::new(UserResponse {
                            id: req.id,
                            username: req.username.clone(),
                            email: req.email.clone(),
                            status: 1,
                        })),
                        Err(e) => {
                            error!("Error: {:?}", e);
                            Err(Status::internal(format!("Failed to update user: {}", e)))
                        }
                    }
                } else {
                    Err(Status::internal(format!("Failed to update user: {}", e)))
                }
            }
        }
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(_) => self.get_user_service_db_connection().await?,
        };

        let user_check = self
            .get_user(Request::new(GetUserRequest { id: req.id }))
            .await;
        if user_check.is_err() {
            error!("User not found");
            return Err(Status::not_found("User not found"));
        }

        let delete_result = userEntity::Entity::delete_by_id(req.id)
            .exec(&db_conn)
            .await;
        match delete_result {
            Ok(result) => {
                if result.rows_affected <= 0 {
                    return Err(Status::internal(format!(
                        "Rows affected: {}",
                        result.rows_affected
                    )));
                }

                Ok(Response::new(UserResponse {
                    id: req.id,
                    username: "".to_string(),
                    email: "".to_string(),
                    status: 1,
                }))
            }
            Err(e) => {
                error!("Error occurred: {:?}", e);
                if let DbErr::Conn(_) = e {
                    let db_conn = self.get_user_service_db_connection().await?;
                    let delete_result = userEntity::Entity::delete_by_id(req.id)
                        .exec(&db_conn)
                        .await;
                    match delete_result {
                        Ok(_) => Ok(Response::new(UserResponse {
                            id: req.id,
                            username: "".to_string(),
                            email: "".to_string(),
                            status: 1,
                        })),
                        Err(e) => Err(Status::internal(format!("Failed to delete user: {}", e))),
                    }
                } else {
                    Err(Status::internal(format!("Failed to delete user: {}", e)))
                }
            }
        }
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner().id.clone();
        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(e) => {
                error!("Error: {:?}", e);
                return Err(Status::internal("Failed to get db connection"));
            }
        };

        let user = userEntity::Entity::find_by_id(req).one(&db_conn).await;
        match user {
            Ok(optional_model) => match optional_model {
                Some(user) => Ok(Response::new(UserResponse {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                    status: 0,
                })),
                None => Err(Status::not_found("User not found")),
            },
            Err(err) => {
                info!("Get user Error: {:?}", err);
                Err(Status::internal(format!("Failed to get user: {}", err)))
            }
        }
    }
    async fn user_by_username_exists(&self, username: String) -> Result<bool, Status> {
        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(e) => {
                error!("Error: {:?}", e);
                return Err(Status::internal("Failed to get db connection"));
            }
        };

        let user = userEntity::Entity::find()
            .filter(user::Column::Username.eq(username.clone()))
            .one(&db_conn)
            .await;
        match user {
            Ok(optional_model) => match optional_model {
                Some(_user) => Ok(true),
                None => Ok(false),
            },
            Err(err) => {
                info!("Get user Error: {:?}", err);
                Err(Status::internal(format!("Failed to get user: {}", err)))
            }
        }
    }
}
