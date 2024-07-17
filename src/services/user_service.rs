use std::sync::Arc;
use tokio::sync::Mutex;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, DbErr, ActiveValue::Set, Database, ActiveValue};
use tonic::{Request, Response, Status};
use crate::userProto::{UserResponse, CreateUserRequest};
use crate::userProto::user_service_server::UserService;
use crate::entities::user as userEntity;
use crate::generated::user::{DeleteUserRequest, GetUserRequest, UpdateUserRequest};

#[derive(Debug, Default)]
pub struct MyUserService {
    pub(crate) db: Arc<Mutex<Option<DatabaseConnection>>>,
}

impl MyUserService {
    pub fn new_user_service(db: Arc<Mutex<Option<DatabaseConnection>>>) -> MyUserService {
        MyUserService { db }
    }

    async fn get_user_service_db_connection(&self) -> Result<DatabaseConnection, Status> {
        let db_guard = self.db.lock().await;
        if let Some(ref db_conn) = *db_guard {
            Ok(db_conn.clone())
        } else {
            Err(Status::internal("Database connection is not available"))
        }
    }

    async fn reconnect_user_service_db(&self) -> Result<(), Status> {
        let mut db_guard = self.db.lock().await;
        match Database::connect("mysql://drosteofficial:adi.2002@162.55.212.205:3306/testAdrian").await {
            Ok(new_db_conn) => {
                *db_guard = Some(new_db_conn);
                Ok(())
            }
            Err(e) => Err(Status::internal(format!("Failed to reconnect to database: {}", e))),
        }
    }
}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        if req.username.is_empty() || req.password.is_empty() || req.email.is_empty() || req.gender < 0 || req.gender > 2 {
            return Err(Status::invalid_argument("Fields cannot be empty"));
        }

        let new_user = userEntity::ActiveModel {
            id: ActiveValue::NotSet,
            username: Set(req.username.clone()),
            password: Set(req.password.clone()),
            email: Set(req.email.clone()),
            gender: Set(req.gender.into()),
        };

        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(_) => {
                self.reconnect_user_service_db().await?;
                self.get_user_service_db_connection().await?
            }
        };

        let insert_result = userEntity::Entity::insert(new_user.clone()).exec(&db_conn).await;
        match insert_result {
            Ok(result) => {
                println!("User initializing...");
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
                    self.reconnect_user_service_db().await?;
                    let db_conn = self.get_user_service_db_connection().await?;
                    let insert_result = userEntity::Entity::insert(new_user.clone()).exec(&db_conn).await;
                    match insert_result {
                        Ok(result) => Ok(Response::new(UserResponse {
                            id: result.last_insert_id as i32,
                            username: req.username.clone(),
                            email: req.email.clone(),
                            status: 1,
                        })),
                        Err(e) => Err(Status::internal(format!("Failed to create user: {}", e))),
                    }
                } else {
                    Err(Status::internal(format!("Failed to create user: {}", e)))
                }
            }
        }
    }

    async fn update_user(&self, request: Request<UpdateUserRequest>) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        if req.username.is_empty() || req.password.is_empty() || req.email.is_empty() || req.gender < 0 || req.gender > 2 {
            return Err(Status::invalid_argument("Fields cannot be empty"));
        }

        let db_conn = match self.get_user_service_db_connection().await {
            Ok(db) => db,
            Err(_) => {
                self.reconnect_user_service_db().await?;
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
                println!("User updated");
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
                    self.reconnect_user_service_db().await?;
                    let db_conn = self.get_user_service_db_connection().await?;
                    let update_result = userEntity::Entity::update(updated_user.clone()).exec(&db_conn).await;
                    match update_result {
                        Ok(_) => Ok(Response::new(UserResponse {
                            id: req.id,
                            username: req.username.clone(),
                            email: req.email.clone(),
                            status: 1,
                        })),
                        Err(e) => Err(Status::internal(format!("Failed to update user: {}", e))),
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
                self.reconnect_user_service_db().await?;
                self.get_user_service_db_connection().await?
            }
        };

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
                    self.reconnect_user_service_db().await?;
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
            Err(_) => {
                self.reconnect_user_service_db().await?;
                self.get_user_service_db_connection().await?
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
                    status: 1,
                }))
            }
            Ok(None) => Err(Status::not_found("User not found")),
            Err(e) => {
                println!("Error: {:?}", e);
                if let DbErr::Conn(_) = e {
                    self.reconnect_user_service_db().await?;
                    let db_conn = self.get_user_service_db_connection().await?;
                    let user = userEntity::Entity::find_by_id(req.id).one(&db_conn).await;
                    match user {
                        Ok(Some(user)) => Ok(Response::new(UserResponse {
                            id: user.id,
                            username: user.username,
                            email: user.email,
                            status: 1,
                        })),
                        Ok(None) => Err(Status::not_found("User not found")),
                        Err(e) => Err(Status::internal(format!("Failed to retrieve user: {}", e))),
                    }
                } else {
                    Err(Status::internal(format!("Failed to retrieve user: {}", e)))
                }
            }
        }
    }
}
