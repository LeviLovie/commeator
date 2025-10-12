use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251011_133950_users::Users;
use crate::m20251011_135939_chats::Chats;

#[derive(DeriveIden)]
pub enum ChatMembers {
    Table,
    ChatId,
    UserId,
    JoinedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChatMembers::Table)
                    .if_not_exists()
                    .col(integer(ChatMembers::ChatId).not_null())
                    .col(integer(ChatMembers::UserId).not_null())
                    .col(
                        timestamp(ChatMembers::JoinedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-chat_members")
                            .col(ChatMembers::ChatId)
                            .col(ChatMembers::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat_members-chat")
                            .from(ChatMembers::Table, ChatMembers::ChatId)
                            .to(Chats::Table, Chats::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat_members-user")
                            .from(ChatMembers::Table, ChatMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChatMembers::Table).to_owned())
            .await
    }
}
