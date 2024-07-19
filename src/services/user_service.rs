
use env_logger::builder;
use crate::userProto::SignInResponse;
use jsonwebtoken::{encode, Header};
use log::{debug, error, log_enabled, info, Level};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, DbErr, ActiveValue::Set, Database, ActiveValue, QuerySelect};
use tonic::{async_trait, Request, Response, Status};
use crate::userProto::{UserResponse, CreateUserRequest};
use crate::userProto::user_service_server::UserService;
use crate::entities::user as userEntity;
use crate::entities::user;
use crate::entities::user::Entity;
use crate::generated::user::{DeleteUserRequest, GetUserRequest, UpdateUserRequest, Response as UResponse, SignInRequest, SignOutRequest};
use sea_orm::ColumnTrait;
use crate::generated::user as genuser;
use crate::entities::RegularToken;
use crate::entities::RefreshToken;
use crate::services::JWTService;
use sea_orm::QueryFilter;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Local, Utc};
use chrono::naive::NaiveDateTime;
use chrono::NaiveDate;
use tonic::metadata::MetadataMap;
use crate::entities::RefreshToken::Column::Token;
use crate::migrations::migrator::sea_query::ColumnRef::Column;

#[derive(Debug, Default)]
pub struct MyUserService {
    pub(crate) db: DatabaseConnection,
    pub(crate)jwt_service: JWTService::JWTService,
}

#[async_trait]
impl UserService for MyUserService {
    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<UResponse>, Status> {
        JWTService::is_token_valid(&request.metadata(), &Default::default()).await;
        self.create_user(request).await
    }

    async fn update_user(&self, request: Request<UpdateUserRequest>) -> Result<Response<UserResponse>, Status> {
        self.update_user(request).await
    }

    async fn delete_user(&self, request: Request<DeleteUserRequest>) -> Result<Response<UserResponse>, Status> {
        self.delete_user(request).await
    }

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<UserResponse>, Status> {
        self.get_user(request).await
    }

    async fn sign_in(&self, request: Request<SignInRequest>) -> Result<Response<SignInResponse>, Status> {
        self.sign_in(request).await
    }

    async fn sign_out(&self, request: Request<SignOutRequest>) -> Result<Response<UResponse>, Status> {
        todo!()
    }
}



impl MyUserService {
    pub fn new_user_service(db: DatabaseConnection, jwt_service:JWTService::JWTService ) -> MyUserService {
        MyUserService { db, jwt_service}

    }

    async fn get_user_service_db_connection(&self) -> Result<DatabaseConnection, Status> {
        Ok(self.db.clone())
    }
}


impl MyUserService {


    async fn sign_out(&self, request: Request<SignOutRequest>, metadata:MetadataMap) -> Result<Response<UResponse>, Status> {
        let DB = self.get_user_service_db_connection().await?;
        let _result = RegularToken::Entity::find()
            .filter(RegularToken::Column::Token.eq(metadata.clone().into_headers().get("authorization").unwrap().to_str().unwrap()))
            .one(&DB).await;
        match _result {
            Ok(Some(token)) => {
                let _ = RegularToken::Entity::update_many()
                    .filter(RegularToken::Column::Token.eq(token.token.clone()))
                    .set(Column::eq(RegularToken::Column::Active.eq(false)))
                    .set(Column::eq(RegularToken::Column::Expirationtime.eq(Utc::now())))
                    .exec(&DB).await;
                match _result.iter().clone() {
                    Ok(_) => {
                        Ok(Response::new(UResponse {
                            status: true,
                            message: "Successfully signed out".to_string(),
                        }))
                    }
                    Err(e) => {
                        error!("Error: {:?}", e);
                        Err(Status::internal(format!("Failed to sign out: {}", e)))
                    }
                }
            }
            Ok(None) => {
                Err(Status::not_found("Token not found"))
            }
            Err(e) => {
                error!("Error: {:?}", e);
                Err(Status::internal(format!("Failed to sign out: {}", e)))
            }
        }
    }


    async fn sign_in(&self, request: Request<SignInRequest>) -> Result<Response<SignInResponse>, Status> {
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
            .filter(user::Column::Username.eq(req.login_or_email.clone()))
            .filter(user::Column::Password.eq(req.password.clone()))
            .one(&db_conn).await;

        match user_result {
            Err(e) => {
                error!("Error: {:?}", e);
                return Err(Status::internal(format!("Failed to sign in: {}", e)));
            },
            Ok(None) => return Err(Status::not_found("User not found")),
            Ok(Some(user)) => {
                let token_result = RegularToken::Entity::find()
                    .filter(RegularToken::Column::Id.eq(user.id))
                    .filter(RegularToken::Column::Active.eq(true))
                    .one(&db_conn).await;

                match token_result {
                    Err(e) => {
                        let new_jwt = JWTService::generate_jwt(user.clone()).await;
                        let new_token = RegularToken::ActiveModel {
                            id: ActiveValue::NotSet,
                            userId: sea_orm::Set(user.id),
                            token: Set(new_jwt.clone()),
                            active: Set(true),
                            creaitontime: Set(NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)),
                            expired: Set(false),
                            expirationtime: ActiveValue::NotSet
                        };
                        RegularToken::Entity::insert(new_token.clone()).exec(&db_conn).await;

                        Ok(Response::new(SignInResponse{
                            status: true,
                            message: "Successfully signed in :)".to_string(),
                            token: new_token.token.unwrap(),
                        }))


                    },
                    Ok(None) => return Err(Status::not_found("Token not found")),
                    Ok(Some(token)) => {
                        return Ok(Response::new(SignInResponse{
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

        if req.username.is_empty() || req.password.is_empty() || req.email.is_empty() || req.gender < 0 || req.gender > 2 {
            return Err(Status::internal("Fields cannot be empty"));
        }

        let new_user = userEntity::ActiveModel {
            id: ActiveValue::NotSet,
            username: Set(req.username.clone()),
            password: Set(req.password.clone()),
            email: Set(req.email.clone()),
            gender: Set(req.gender.into()),
        };


        let insert_result = userEntity::Entity::insert(new_user.clone()).exec(&self.db).await;
        match insert_result {
            Ok(result) => {
                info!("User created");
                Ok(Response::new(UResponse {
                    status: true,
                    message: "".to_string(),
                }))
            }
            Err(e) => {
                println!("Error: {:?}", e);
                if let DbErr::Conn(_) = e {
                    let db_conn = self.get_user_service_db_connection().await?;
                    let insert_result = userEntity::Entity::insert(new_user.clone()).exec(&db_conn).await;
                    match insert_result {
                        Ok(result) => Ok(Response::new(UResponse {
                            status: true,
                            message: "successfully Registered".to_string(),
                        })),
                        Err(e) => {
                            error!("Error: {:?}", e);
                            Err(Status::internal(format!("Failed to create user: {}", e)))
                        }
                    }
                } else {
                    error!("Error: {:?}", e);
                    Err(Status::internal(format!("Failed to create user: {}", e)))
                }
            }
        }
    }

    async fn update_user(&self, request: Request<UpdateUserRequest>) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        if req.username.is_empty() || req.password.is_empty() || req.email.is_empty() || req.gender < 0 || req.gender > 2 {
            error!("Fields cannot be empty");
            return Err(Status::invalid_argument("Fields cannot be empty"));
        }

        let user_check = self.get_user(Request::new(GetUserRequest { id: req.id })).await;
        if user_check.is_err() {
            error!("User not found");
            return Err(Status::not_found("User not found"));
        }

        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(_) => {
                self.get_user_service_db_connection().await?
            }
        };

        let updated_user = userEntity::ActiveModel {
            id: ActiveValue::Set(req.id),
            username: Set(req.username.clone()),
            password: Set(req.password.clone()),
            email: Set(req.email.clone()),
            gender: Set(req.gender.into()),
        };

        let update_result = userEntity::Entity::update(updated_user.clone()).exec(&db_conn).await;
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
                println!("Error: {:?}", e);
                if let DbErr::Conn(_) = e {
                    let db_conn = self.get_user_service_db_connection().await?;
                    let update_result = userEntity::Entity::update(updated_user.clone()).exec(&db_conn).await;
                    match update_result {
                        Ok(_) => Ok(Response::new(UserResponse {
                            id: req.id,
                            username: req.username.clone(),
                            email: req.email.clone(),
                            status: 1,
                        })),
                        Err(e) =>{
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

    async fn delete_user(&self, request: Request<DeleteUserRequest>) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        println!("Request: {:?}", req);
        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(_) => {
                self.get_user_service_db_connection().await?
            }
        };

        let user_check = self.get_user(Request::new(GetUserRequest { id: req.id })).await;
        if user_check.is_err() {
            error!("User not found");
            return Err(Status::not_found("User not found"));
        }

        let delete_result = userEntity::Entity::delete_by_id(req.id).exec(&db_conn).await;
        match delete_result {
            Ok(result) => {
                if result.rows_affected <= 0 {
                    return Err(Status::internal(format!("Rows affected: {}", result.rows_affected)));
                }

                println!("User deleted");
                Ok(Response::new(UserResponse {
                    id: req.id,
                    username: "".to_string(),
                    email: "".to_string(),
                    status: 1,
                }))
            }
            Err(e) => {
                println!("Error occurred: {:?}", e);
                if let DbErr::Conn(_) = e {
                    let db_conn = self.get_user_service_db_connection().await?;
                    let delete_result = userEntity::Entity::delete_by_id(req.id).exec(&db_conn).await;
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

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();
        println!("Request: {:?}", req);
        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(Status::internal("Failed to get db connection"));
            }
        };

        let user = userEntity::Entity::find_by_id(req.id).one(&db_conn).await;
        match user {
            Ok(Some(user)) => {
                println!("User retrieved");
                Ok(Response::new(UserResponse {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                    status: 0,
                }))
            }
            Ok(None) => Err(Status::not_found("User not found")),
            Err(e) => { Err(Status::internal(format!("Failed to get user: {}", e))) }
        }
    }
}


