use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Chats {
    Table,
    UUID,
    Name,
    IsGroup,
    CreatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chats::Table)
                    .if_not_exists()
                    .col(
                        uuid(Chats::UUID)
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()"))
                    )
                    .col(string(Chats::Name).not_null())
                    .col(boolean(Chats::IsGroup).not_null())
                    .col(
                        timestamp(Chats::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chats::Table).to_owned())
            .await
    }
}
