use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Username)
                            .string()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::Password).string().null())
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::Gender).integer().null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).if_exists().to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Password,
    Email,
    Gender,
}
