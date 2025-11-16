use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20251011_133950_users::Users, m20251011_135939_chats::Chats};

#[derive(DeriveIden)]
enum ChatNames {
    Table,
    ChatUuid,
    UserUuid,
    Name,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChatNames::Table)
                    .if_not_exists()
                    .col(
                        uuid(ChatNames::ChatUuid)
                            .not_null(),
                    )
                    .col(
                        uuid(ChatNames::UserUuid)
                            .not_null(),
                    )
                    .col(
                        string(ChatNames::Name)
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk-chat_names")
                            .col(ChatNames::ChatUuid)
                            .col(ChatNames::UserUuid),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat_names-chat")
                            .from(ChatNames::Table, ChatNames::ChatUuid)
                            .to(Chats::Table, Chats::Uuid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-chat_names-user")
                            .from(ChatNames::Table, ChatNames::UserUuid)
                            .to(Users::Table, Users::Uuid)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ChatNames::Table).to_owned())
            .await
    }
}
