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
    use anyhow::{Context, Result, bail};
    use reqwest::Client;

    use super::*;
    use crate::config::{endpoints::auth::URI_WHOAMI, on_auth_base_url};

    pub async fn get_user_from_cookie(cookie: &str) -> Result<KratosUserData> {
        let res = Client::new()
            .get(on_auth_base_url(URI_WHOAMI))
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
