use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct KratosUserData {
    pub identity: Identity,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Identity {
    pub traits: Traits,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Traits {
    pub email: String,
}

#[cfg(feature = "server")]
pub mod server {
    use anyhow::{bail, Context, Result};
    use reqwest::Client;

    use crate::config::auth::URL_WHOAMI;
    use super::*;

    pub async fn get_user_from_cookie(cookie: &str) -> Result<KratosUserData> {
        let res = Client::new()
            .get(URL_WHOAMI)
            .header("Cookie", cookie)
            .send()
            .await?;

        if !res.status().is_success() {
            bail!("Failed to verify session with Kratos: {}", res.status());
        }

        res.json::<KratosUserData>()
            .await
            .context("Failed to parse user info from Kratos")
    }
}
