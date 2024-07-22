use std::env;

use dotenv::dotenv;
use log::info;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tonic::transport::Server;

use generated::calculator;
use generated::pow;
use generated::user as userProto;

use crate::generated::messages::messages_server::MessagesServer;
use crate::services::calculator_service::CalculatorService;
use crate::services::jwt_service;
use crate::services::messages_service;
use crate::services::pow_service::PowService;
use crate::services::user_service::MyUserService;

mod entities;
mod services;
//mod client;
mod generated;
mod migrations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv().ok();
    info!("Starting server");
    match env::var("DATABASE_URL") {
        Ok(db_url) => info!("DATABASE_URL: {}", db_url),
        Err(e) => eprintln!("Error loading DATABASE_URL: {}", e),
    }

    //init. logger
    // Create database
    println!("Initializing database connection");
    let db_conn: DatabaseConnection =
        Database::connect("mysql://drosteofficial:adi.2002@162.55.212.205/testAdrian").await?;

    //Initialize JWTService
    let jwtservice = jwt_service::JWTService::new(db_conn.clone());

    // Run migrations
    migrations::migrator::Migrator::up(&db_conn, None).await?;

    let addr = "[::1]:5051".parse()?;
    println!("Building server");
    Server::builder()
        .add_service(calculator::calculator_server::CalculatorServer::new(
            CalculatorService::default(),
        ))
        .add_service(pow::pow_server::PowServer::new(PowService::default()))
        .add_service(userProto::user_service_server::UserServiceServer::new(
            MyUserService::new_user_service(db_conn.clone(), jwtservice),
        ))
        .add_service(MessagesServer::new(
            messages_service::MessagesService::new_messages_service(db_conn.clone()),
        ))
        .serve(addr)
        .await?;

    println!("Server initialized at {}", addr);
    Ok(())
}
