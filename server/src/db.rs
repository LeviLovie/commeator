use async_once_cell::OnceCell;
use sea_orm::{Database, DatabaseConnection};
use utils::config::server::db_url;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn db() -> &'static DatabaseConnection {
    DB.get_or_init(async {
        Database::connect(db_url())
            .await
            .expect("Failed to connect to database")
    })
    .await
}
