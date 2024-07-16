mod DBManager;
mod auth;
mod Entities;
mod services;

use sea_orm::{Database, DatabaseConnection, DbErr, ModelTrait};
use tonic::transport::Server;
use crate::services::calculator_service::CalculatorService;
use crate::services::pow_service::PowService;

mod calculator {
    tonic::include_proto!("calculator");
}

mod pow {
    tonic::include_proto!("pow");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Assuming `Database::connect` returns a `DatabaseConnection` that can be used to create a `DBManager` instance
    let db_manager = DBManager::DBManager {
        db: Database::connect("mysql://drosteofficial:adi.2002@162.55.212.205:3306/testAdrian").await?,
    };

    let user = Entities::user::Model {
        id: 0,
        username: "test_username".to_string(),
        password: "test_password".to_string(),
        email: "test_email@example.com".to_string(),
        hashed_password: "hashed_test_password".to_string(),
    };

    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(calculator::calculator_server::CalculatorServer::new(CalculatorService::default()))
        .add_service(pow::pow_server::PowServer::new(PowService::default()))
        .serve(addr)
        .await?;

    Ok(())
}