pub mod users;
pub mod chats;
pub mod jwt;

#[cfg(feature = "server")]
mod verify_kratos;

#[cfg(feature = "server")]
#[allow(unused_imports)]
mod entities;

#[cfg(feature = "server")]
mod server_utils {
    pub use super::entities::prelude::*;
    pub use super::entities::*;
    pub use super::verify_kratos::verify_kratos_cookie;
    pub use super::jwt::verify_jwt;

    pub use sea_orm::{ActiveModelTrait, ActiveValue::*, ColumnTrait, EntityTrait, QueryFilter};

    use async_once_cell::OnceCell;
    use sea_orm::{Database, DatabaseConnection};

    use crate::config::server_utils::DATABASE_URL;

    static DB: OnceCell<DatabaseConnection> = OnceCell::new();

    pub async fn db() -> &'static DatabaseConnection {
        DB.get_or_init(async {
            Database::connect(DATABASE_URL)
                .await
                .expect("Failed to connect to database")
        })
        .await
    }
}
