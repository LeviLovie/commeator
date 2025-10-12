pub mod users;

#[cfg(feature = "server")]
#[allow(unused_imports)]
mod entities;

#[cfg(feature = "server")]
mod server_utils {
    pub use super::entities::*;
    pub use super::entities::prelude::*;
    pub use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveValue::*, ActiveModelTrait};

    use sea_orm::{Database, DatabaseConnection};
    use async_once_cell::OnceCell;

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
