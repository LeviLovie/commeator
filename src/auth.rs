use anyhow::{bail, Context, Result};
use dioxus::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

use crate::config::{URL_LOGIN, URL_WHOAMI};

#[derive(Deserialize, Debug, Clone)]
pub struct UserInfo {
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

async fn try_get_user() -> Result<UserInfo> {
    match Request::get(URL_WHOAMI)
        .credentials(web_sys::RequestCredentials::Include)
        .send()
        .await
        .context("Failed to send request to Kratos")?
    {
        r if r.ok() => r
            .json::<UserInfo>()
            .await
            .context("Failed to parse user info from Kratos"),
        r => Err(anyhow::anyhow!("Received non-OK response: {}", r.status())),
    }
}

#[cfg(feature = "server")]
pub async fn get_user_from_cookie(cookie: &str) -> Result<UserInfo> {
    let client = dioxus_fullstack::reqwest::Client::new();
    let res = client
        .get(URL_WHOAMI)
        .header("Cookie", cookie)
        .send()
        .await?;

    if !res.status().is_success() {
        bail!("Failed to verify session with Kratos: {}", res.status());
    }

    res.json::<UserInfo>()
        .await
        .context("Failed to parse user info from Kratos")
}

pub async fn get_user() -> Option<UserInfo> {
    match try_get_user().await {
        Ok(user) => Some(user),
        Err(e) => {
            error!("Error fetching user info: {}", e);
            navigator().replace(URL_LOGIN);
            None
        }
    }
}
