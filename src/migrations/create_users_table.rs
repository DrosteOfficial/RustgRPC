use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(User::username)
                        .string()
                        .default(false)
                        .not_null(),
                )
                .col(
                    ColumnDef::new(User::password)
                        .timestamp()
                        .null(),
                )
                .col(
                    ColumnDef::new(User::email)
                        .string()
                        .default(false)
                        .not_null(),
                )
                .col(
                    ColumnDef::new(User::gender)
                        .timestamp()
                        .null(),
                )

                .to_owned()
        )
            .await
    }
async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).if_exists().to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum User {
    Table,
    id,
    username,
    password,
    email,
    gender,
}

#[derive(DeriveIden)]
#[warn(dead_code)]
enum Messages{
    Table,
    id,
    message,
    sender,
    receiver,
    timestamp,
}

