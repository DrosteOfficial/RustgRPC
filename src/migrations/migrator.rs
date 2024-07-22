use sea_orm_migration::async_trait::async_trait;
pub use sea_orm_migration::*;

use crate::migrations::create_messages_table as messageMigration;
use crate::migrations::create_regular_token as regularTokenMigration;
use crate::migrations::create_users_table as userMigration;

pub struct Migrator;
#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(userMigration::Migration),
            Box::new(messageMigration::CreateMessagesTable),
            Box::new(regularTokenMigration::Migration),
        ]
    }
}
