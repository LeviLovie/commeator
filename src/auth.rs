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

#[macro_export]
macro_rules! user {
    () => {
        {
            let user = use_resource(|| async { $crate::auth::get_user().await });
            if user().is_none() || user().as_ref().unwrap().is_none() {
                return rsx! { $crate::components::Spinner {} };
            }
            user().as_ref().unwrap().as_ref().unwrap().clone();
        };
    };
}

#[macro_export]
macro_rules! verify_user_jwt {
    () => {
        {
            let user = use_resource(|| async { $crate::auth::get_user().await });
            if user().is_none() || user().as_ref().unwrap().is_none() {
                return rsx! { $crate::components::Spinner {} };
            }
            let user = user().as_ref().unwrap().as_ref().unwrap().clone();

            let jwt = use_resource(|| async { $crate::pages::state::jwt().await });
            if jwt().is_none() {
                return rsx! { $crate::components::Spinner {} };
            }
            let jwt = jwt();
            let jwt_clone = jwt.clone();
            let is_valid = use_resource(move || {
                let jwt_clone = jwt_clone.clone();
                async move  {
                    if let Some(token) = jwt_clone {
                        if token == "NO_JWT" {
                            false
                        } else {
                            $crate::backend::jwt::verify_jwt_endpoint(token).await.unwrap_or(false)
                        }
                    } else {
                        false
                    }
                }
            });
            if is_valid().is_none() {
                return rsx! { $crate::components::Spinner {} };
            }
            let is_valid = is_valid();
            if !is_valid.unwrap() {
                warn!("Invalid JWT for user {}", user.identity.traits.email);
                spawn(async {
                    if let Err(e) = $crate::pages::state::request_jwt().await {
                        error!("Failed to request new JWT: {}", e);
                        navigator().replace($crate::Route::Home);
                    }
                });
            }

            (user, jwt.clone().unwrap())
        }
    };
}
