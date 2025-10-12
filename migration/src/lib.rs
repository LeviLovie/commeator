pub use sea_orm_migration::prelude::*;

mod m20251011_133950_users;
mod m20251011_135939_chats;
mod m20251011_140310_chat_members;
mod m20251011_141157_messages;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251011_133950_users::Migration),
            Box::new(m20251011_135939_chats::Migration),
            Box::new(m20251011_140310_chat_members::Migration),
            Box::new(m20251011_141157_messages::Migration),
        ]
    }
}
