use sea_orm::prelude::DateTime;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RegularToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RegularToken::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RegularToken::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(RegularToken::Token)
                            .string()
                            .string_len(500)
                            .default("")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RegularToken::Active)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RegularToken::CreationTime)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RegularToken::Expired)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RegularToken::ExpirationTime)
                            .timestamp()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RegularToken::Table, RegularToken::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(RegularToken::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum RegularToken {
    Table,
    Id,
    UserId,
    Token,
    Active,
    CreationTime,
    Expired,
    ExpirationTime,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
