use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct CreateMessagesTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateMessagesTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Messages::Message).string().not_null())
                    .col(ColumnDef::new(Messages::Sender).integer().not_null())
                    .col(ColumnDef::new(Messages::Receiver).integer().not_null())
                    .col(ColumnDef::new(Messages::Timestamp).big_integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Messages::Table, Messages::Sender)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Messages::Table, Messages::Receiver)
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
            .drop_table(Table::drop().table(Messages::Table).if_exists().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Messages {
    Table,
    Id,
    Message,
    Sender,
    Receiver,
    Timestamp,
}
#[warn(dead_code)]
#[derive(Iden)]
enum Users {
    Table,
    Id,
}
