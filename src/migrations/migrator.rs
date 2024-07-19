pub use sea_orm_migration::*;
use sea_orm_migration::async_trait::async_trait;
use crate::migrations::create_users_table as userMigration;
use crate::migrations::create_messages_table as messageMigration;

pub struct Migrator;
#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(userMigration::Migration),
            Box::new(messageMigration::CreateMessagesTable),
        ]
    }
}