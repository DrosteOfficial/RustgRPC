mod DBManager;
mod auth;
mod Entities;
mod services;
mod Client;
mod generated;
use generated::calculator;
use generated::pow;
use generated::messages;
use generated::user as userProto;



use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, EntityName, EntityOrSelect, ModelTrait, Schema};
use tonic::transport::Server;
use crate::services::calculator_service::CalculatorService;
use crate::services::pow_service::PowService;
use crate::userProto::user_service_server::{ UserService};
use sqlx::Statement;
use crate::Entities::user::Entity as UserEntity;
use crate::generated::messages::messages_server::MessagesServer;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_conn_lock = DatabaseConnection::lock().unwrap();
    let db_connection = &db_conn_lock.connection;
    println!("Connected to database {}", db_connection.get_database_backend().get_uri());
    // create_tables(&database_connection).await?;

    let user = Entities::user::Model {
        id: 1,
        username: "test_username".to_string(),
        password: "test_password".to_string(),
        email: "test_email@example.com".to_string(),
        gender: Entities::user::GenderTypes::Male,
    };

    let addr = "[::1]:5051".parse()?;

    Server::builder()
        .add_service(calculator::calculator_server::CalculatorServer::new(CalculatorService::default()))
        .add_service(pow::pow_server::PowServer::new(PowService::default()))
        .add_service(userProto::user_service_server::UserServiceServer::new(services::user_service::MyUserService::default()))
        .add_service(MessagesServer::new(services::messages_service::MyMessagesService::default()))
        .serve(addr)
        .await?;

    Ok(())
}



async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let entities = vec![
        UserEntity,
    ];

    for entity in entities {
        let table_name = entity.table_name();
        println!("Checking table: {}", table_name);
        if entity.select().one(db).await? != None {
            println!("Table {} already exists", table_name);
            continue;
        }
    }

    Ok(())
}