use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251011_133950_users::Users;
use crate::m20251011_135939_chats::Chats;

#[derive(DeriveIden)]
pub enum Messages {
    Table,
    UUID,
    ChatUUID,
    SenderUUID,
    SenderNickName,
    Content,
    CreatedAt,
    EditedAt,
    Deleted,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        uuid(Messages::UUID)
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()"))
                    )
                    .col(uuid(Messages::ChatUUID).not_null())
                    .col(uuid(Messages::SenderUUID).not_null())
                    .col(string(Messages::SenderNickName).not_null())
                    .col(text(Messages::Content).not_null())
                    .col(
                        timestamp(Messages::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(timestamp_null(Messages::EditedAt))
                    .col(boolean(Messages::Deleted).not_null().default(false))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-chat")
                            .from(Messages::Table, Messages::ChatUUID)
                            .to(Chats::Table, Chats::UUID)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-messages-sender")
                            .from(Messages::Table, Messages::SenderUUID)
                            .to(Users::Table, Users::UUID)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await
    }
}
