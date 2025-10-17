use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251011_133950_users::Users;
use crate::m20251011_135939_chats::Chats;

#[derive(DeriveIden)]
pub enum ChatMembers {
    Table,
    ChatUUID,
    UserUUID,
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
                    .col(
                        uuid(ChatMembers::ChatUUID)
                            .not_null(),
                    )
                    .col(
                        uuid(ChatMembers::UserUUID)
                            .not_null(),
                    )
                    .col(
                        timestamp(ChatMembers::JoinedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-chat_members")
                            .col(ChatMembers::ChatUUID)
                            .col(ChatMembers::UserUUID),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat_members-chat")
                            .from(ChatMembers::Table, ChatMembers::ChatUUID)
                            .to(Chats::Table, Chats::UUID)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat_members-user")
                            .from(ChatMembers::Table, ChatMembers::UserUUID)
                            .to(Users::Table, Users::UUID)
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
