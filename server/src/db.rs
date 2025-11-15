use anyhow::{Context, Result};
use sea_orm::{Database, DatabaseConnection};

use crate::config::CONFIG;

pub struct Db(pub DatabaseConnection);

pub async fn init() -> Result<Db> {
    let conn = Database::connect(CONFIG.db_url.clone())
        .await
        .context("Failed to connect to the database")?;

    Ok(Db(conn))
}
