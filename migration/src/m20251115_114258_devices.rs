use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251011_133950_users::Users;

#[derive(DeriveIden)]
enum Devices {
    Table,
    Uuid,
    UserUuid,
    Jwt,
    Os,
    AuthenticatedAt,
    LastOnline,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Devices::Table)
                    .if_not_exists()
                    .col(
                        uuid(Devices::Uuid)
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuid_generate_v4()"))
                    )
                    .col(uuid(Devices::UserUuid).not_null())
                    .col(text(Devices::Jwt).not_null())
                    .col(text(Devices::Os).not_null())
                    .col(
                        timestamp(Devices::AuthenticatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Devices::LastOnline)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-devices-user")
                            .from(Devices::Table, Devices::UserUuid)
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
            .drop_table(Table::drop().table(Devices::Table).to_owned())
            .await
    }
}
