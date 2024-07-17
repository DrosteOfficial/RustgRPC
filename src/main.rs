mod dbmanager;
mod auth;
mod entities;
mod services;
mod client;
mod generated;

use tokio::sync::Mutex;
use std::sync::Arc;
use generated::calculator;
use generated::pow;
use generated::user as userProto;

use sea_orm::{Database, DatabaseConnection};
use tonic::transport::Server;
use crate::services::calculator_service::CalculatorService;
use crate::services::pow_service::PowService;
use crate::generated::messages::messages_server::MessagesServer;
use crate::services::user_service::MyUserService;
use crate::services::messages_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing database connection");
    let db_conn: DatabaseConnection = Database::connect("mysql://drosteofficial:adi.2002@162.55.212.205/testAdrian").await?;
    let db_conn_arc = Arc::new(Mutex::new(Some(db_conn)));

    println!("Initializing user service");
    let user_service = MyUserService::new_user_service(db_conn_arc.clone());

    println!("Initializing messages service");


    let addr = "[::1]:5051".parse()?;
    println!("Building server");
    Server::builder()
        .add_service(calculator::calculator_server::CalculatorServer::new(CalculatorService::default()))
        .add_service(pow::pow_server::PowServer::new(PowService::default()))
        .add_service(userProto::user_service_server::UserServiceServer::new(user_service))
        .add_service(MessagesServer::new(messages_service::MessagesService::new_messages_service(db_conn_arc.clone())))
        .serve(addr)
        .await?;

    println!("Server initialized at {}", addr);
    Ok(())
}
