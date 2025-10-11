use gloo_net::http::Request;
use dioxus::prelude::*;
use serde::Deserialize;
use anyhow::{Context, Result};

use crate::config::{URL_WHOAMI, URL_LOGIN};

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
        .await.context("Failed to send request to Kratos")? {
            r if r.ok() => {
                r.json::<UserInfo>()
                    .await
                    .context("Failed to parse user info from Kratos")
            }
            r => Err(anyhow::anyhow!("Received non-OK response: {}", r.status())),
        }
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

