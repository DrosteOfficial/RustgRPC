use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, ExecResult, Statement};
use tonic::transport::Server;
use crate::services::calculator_service::CalculatorService;
use crate::services::pow_service::PowService;

mod services {
    pub mod calculator_service;
    pub mod pow_service;
}

mod calculator {
    tonic::include_proto!("calculator");
}

mod pow {
    tonic::include_proto!("pow");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db: DatabaseConnection = Database::connect("mysql://drosteofficial:adi.2002@162.55.212.205:3306/testAdrian").await?;
    create_table_if_not_exists(&db).await?;
    check_db_connection(&db).await?;

    let addr = "[::1]:50051".parse()?;
    let calculator_service = CalculatorService::default();
    let pow_service = PowService::default();

    Server::builder()
        .add_service(calculator::calculator_server::CalculatorServer::new(calculator_service))
        .add_service(pow::pow_server::PowServer::new(pow_service))
        .serve(addr)
        .await?;

    Ok(())
}

async fn create_table_if_not_exists(db: &DatabaseConnection) -> Result<ExecResult, DbErr> {
    let raw_sql = "CREATE TABLE IF NOT EXISTS testAdrian (id INT PRIMARY KEY AUTO_INCREMENT, name TEXT NOT NULL)";

    db.execute(Statement::from_string(db.get_database_backend(), raw_sql)).await
}

async fn check_db_connection(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.ping().await
}