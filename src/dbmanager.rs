// use sea_orm::{entity::*, DatabaseConnection, DbErr, ExecResult, Statement, ConnectionTrait, Database};
// use std::fmt::Write;
// use crate::entities::User;
//
// pub struct DBManager {
//     db: DatabaseConnection,
// }
//
// struct ColumnDef {
//     name: String,
//     column_type: String, // Adjust the type as necessary
// }
//
// impl DBManager {
//     pub async fn new(db_url: &str) -> Result<Self, DbErr> {
//         let db = Database::connect(db_url).await?;
//         Ok(Self { db })
//     }
//
//     pub async fn create_table_generic(&self, table_name: &str) -> Result<ExecResult, DbErr> {
//         let columns = self.get_entity_columns(table_name).await?;
//
//         let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (", table_name);
//         for (i, column) in columns.iter().enumerate() {
//             let column_name = column.name.clone();
//             let column_type = column.column_type.clone().to_string(); // Assuming `column` has a `column_type` method
//             write!(&mut sql, "{} {}", column_name, column_type).unwrap();
//             if i < columns.len() - 1 {
//                 sql.push_str(", ");
//             }
//         }
//         sql.push(')');
//
//         self.db.execute(
//             Statement::from_sql_and_values(
//                 self.db.get_database_backend(),
//                 &sql,
//                 vec![],
//             )
//         ).await
//     }
//
//     async fn get_entity_columns(&self, table_name: &str) -> Result<Vec<ColumnDef>, DbErr> {
//         match table_name {
//             "User" => {
//                 // Manually define columns for the User entity
//                 let columns = vec![
//                     ColumnDef { name: "id".to_string(), column_type: "INTEGER".to_string() },
//                     ColumnDef { name: "username".to_string(), column_type: "VARCHAR(255)".to_string() },
//                     ColumnDef { name: "email".to_string(), column_type: "VARCHAR(255)".to_string() },
//                     // Add more columns as needed
//                 ];
//                 Ok(columns)
//             },
//             // Handle other entities here
//             _ => Err(DbErr::Custom(format!("Table {} not found", table_name))),
//         }
//     }
// }


// use sea_orm::{DatabaseConnection, DbErr, EntityTrait, IntoActiveModel};
// use super::entities as Ent;
// use sea_orm_migration::seaql_migrations::Entity as User;
// pub async fn insert_user(user: &Ent::user::Model, database_connection: &DatabaseConnection) -> Result<(), DbErr> {
//     let active_user = user.clone().into_active_model();
//
//     let result = User::insert(active_user).exec(database_connection).await;
//
//
//     match result {
//         Ok(_) => {
//             Ok(println!("User inserted"))
//         }
//         Err(e) => Err(e),
//     }
// }




