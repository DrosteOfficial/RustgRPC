use std::sync::Arc;
use crate::DatabaseConnection::SqlxMySqlPoolConnection;
use sea_orm::{Database, DatabaseConnection};
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use crate::userProto::{UserResponse, CreateUserRequest, UpdateUserRequest, DeleteUserRequest, GetUserRequest};
use crate::userProto::user_service_server::{UserService};
use crate::Entities::user;

#[derive(Debug, Default)]
pub struct MyUserService {}

#[tonic::async_trait]
impl UserService for MyUserService {

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        // Assuming `DbConn::lock().unwrap()` gives you a `DatabaseConnection`
        let db = Database::connect("mysql://drosteofficial:adi.2002@162.55.212.205:3306/testAdrian").await;

        if req.username.is_empty() || req.password.is_empty() || req.email.is_empty() {
            return Err(Status::invalid_argument("Fields cannot be empty"));
        }

        // Create a new user instance from the request
        let new_user = user::ActiveModel {
            username: Set(req.username),
            password: Set(req.password),
            email: Set(req.email),
            ..Default::default() // Fill in other fields as necessary
        };

        // Insert the new user into the database

        let insert_result = user::Entity::insert(new_user)
            .exec()
            .await;

        match insert_result {
            Ok(_) => Ok(Response::new(UserResponse {
                id: 1,
                username: "".to_string(),
                email: "".to_string(),
                status: 0,
            })),
            Err(e) => Err(Status::internal(format!("Failed to create user: {}", e))),
        }
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        // Implement your business logic here
        unimplemented!()
    }

    async fn delete_user(
        &self,
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        // Implement your business logic here
        unimplemented!()
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        // Implement your business logic here
        unimplemented!()
    }
}