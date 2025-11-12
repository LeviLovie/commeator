use anyhow::{Context, Result};
use sea_orm::{Database, DatabaseConnection};

pub struct Db(pub DatabaseConnection);

pub async fn init() -> Result<Db> {
    let conn = Database::connect(crate::env::database_url())
        .await
        .context("Failed to connect to the database")?;

    Ok(Db(conn))
}
