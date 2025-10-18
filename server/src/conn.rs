use anyhow::anyhow;
use async_once_cell::OnceCell;
use sea_orm::{Database, DatabaseConnection};

use crate::AppError;
use utils::{
    config::server::{centrifugo_key, centrifugo_url, db_url},
    updates::Update,
};

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn db() -> &'static DatabaseConnection {
    DB.get_or_init(async {
        Database::connect(db_url())
            .await
            .expect("Failed to connect to database")
    })
    .await
}

pub async fn publish(channel: &str, update: Update) -> Result<(), AppError> {
    let payload =
        serde_json::to_value(update).map_err(|e| anyhow!("Failed to serialize update: {}", e))?;

    let body = serde_json::json!({
        "channel": channel,
        "data": payload
    });

    let client = reqwest::Client::new();

    let res = client
        .post(format!("{}/api/publish", centrifugo_url()))
        .header("Authorization", format!("apikey {}", centrifugo_key()))
        .json(&body)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        let text = res.text().await?;
        Err(anyhow!("Failed to publish message: {}", text).into())
    }
}
