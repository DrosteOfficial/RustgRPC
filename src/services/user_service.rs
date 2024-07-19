use env_logger::builder;
use crate::userProto::SignInResponse;
use log::{debug, error, log_enabled, info, Level};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, DbErr, ActiveValue::Set, Database, ActiveValue, QueryFilter, QuerySelect, ColumnTrait, QueryTrait, ConnectionTrait, Related, ModelTrait};
use tonic::{Request, Response, Status};
use crate::userProto::{UserResponse, CreateUserRequest};
use crate::userProto::user_service_server::UserService;
use crate::entities::user as userEntity;
use crate::entities::user;
use crate::entities::user::Entity;
use crate::generated::user::{DeleteUserRequest, GetUserRequest, UpdateUserRequest, Response as UResponse, SignInRequest, SignOutRequest};
use crate::generated::user as genuser;
use crate::entities::RegularToken;
use crate::entities::RefreshToken;
use crate::entities::user::Relation::RefreshToken;

#[derive(Debug, Default)]
pub struct MyUserService {
    pub(crate) db: DatabaseConnection,
}


#[tonic::async_trait]
impl UserService for MyUserService {
    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<UserResponse>, Status> {
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
        todo!()
    }

    async fn sign_out(&self, request: Request<SignOutRequest>) -> Result<Response<UResponse>, Status> {
        todo!()
    }
}



impl MyUserService {
    pub fn new_user_service(db: DatabaseConnection) -> MyUserService {
        MyUserService { db }
    }

    async fn get_user_service_db_connection(&self) -> Result<DatabaseConnection, Status> {
        Ok(self.db.clone())
    }
}




impl MyUserService {
    async fn sign_in(&self, request: Request<SignInRequest>) -> Result<Response<SignInResponse>, Status>{
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

        let _result =  userEntity::Entity::find()
            .having(user::Column::Username.eq(req.login_or_email.clone()))
            .having(user::Column::Password.eq(req.password.clone()))
            .one(&db_conn).await;

        match _result {
            Err(e) => {
                error!("Error: {:?}", e);
                return Err(Status::internal(format!("Failed to sign in: {}", e)));
            }

            Ok(Some(user)) => {
                let _result =  user::Entity::find_related(RefreshToken);











       }
    }
}
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
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
                Ok(Response::new(UserResponse {
                    id: result.last_insert_id as i32,
                    username: req.username.clone(),
                    email: req.email.clone(),
                    status: 1,
                }))
            }
            Err(e) => {
                println!("Error: {:?}", e);
                if let DbErr::Conn(_) = e {
                    let db_conn = self.get_user_service_db_connection().await?;
                    let insert_result = userEntity::Entity::insert(new_user.clone()).exec(&db_conn).await;
                    match insert_result {
                        Ok(result) => Ok(Response::new(UserResponse {
                            id: result.last_insert_id as i32,
                            username: req.username.clone(),
                            email: req.email.clone(),
                            status: 1,
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


