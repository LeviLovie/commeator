use chrono::{NaiveDateTime, Utc};
use dioxus::prelude::*;
use std::sync::{LazyLock, Mutex};
use utils::{
    config::{endpoints::jwt::IG_GENERATE_CENTRIFUGO, on_api_base_url},
    requests::GenerateJwtResponse,
};

use crate::{backend::Request, components::logout};

#[cfg(target_arch = "wasm32")]
pub static JWT: LazyLock<Mutex<Option<(String, NaiveDateTime)>>> =
    LazyLock::new(|| Mutex::new(None));

pub static CENTRIFUGO_JWT: LazyLock<Mutex<Option<(String, NaiveDateTime)>>> =
    LazyLock::new(|| Mutex::new(None));

#[cfg(target_arch = "wasm32")]
pub async fn get_jwt() -> Option<String> {
    let jwt = {
        let guard = JWT.lock().unwrap();
        guard.clone()
    };

    match jwt {
        None => regenerate_jwt().await,
        Some((_, expires_at)) if expires_at <= Utc::now().naive_utc() => regenerate_jwt().await,
        _ => {}
    };

    if let Some(token) = JWT.lock().unwrap().as_ref() {
        Some(token.0.clone())
    } else {
        error!("JWT is still None after regeneration attempt");
        None
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_jwt() -> Option<String> {
    super::local_storage::load_jwt()
}

pub async fn get_centrifugo_jwt() -> Option<String> {
    {
        let jwt = {
            let guard = CENTRIFUGO_JWT.lock().unwrap();
            guard.clone()
        };
        match jwt {
            None => regenerate_centrifugo_jwt().await,
            Some((_, expires_at)) if expires_at <= Utc::now().naive_utc() => {
                regenerate_centrifugo_jwt().await
            }
            _ => {}
        }
    };

    if let Some(token) = CENTRIFUGO_JWT.lock().unwrap().as_ref() {
        Some(token.0.clone())
    } else {
        warn!("Centrifugo JWT is still None after regeneration attempt");
        None
    }
}

#[cfg(target_arch = "wasm32")]
async fn regenerate_jwt() {
    match generate_jwt().await {
        Ok(token) => {
            *JWT.lock().unwrap() = Some(token);
        }
        Err(e) => {
            if e.to_string().contains("User not found") {
                logout().await;
            }
            error!("Failed to request JWT: {}", e);
        }
    }
}

async fn regenerate_centrifugo_jwt() {
    match generate_centrifugo_jwt().await {
        Ok(token) => {
            *CENTRIFUGO_JWT.lock().unwrap() = Some(token);
        }
        Err(e) => {
            if e.to_string().contains("User not found") {
                logout().await;
            }
            error!("Failed to request centrifugo JWT: {}", e);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn generate_jwt() -> Result<(String, NaiveDateTime)> {
    let response = Request::get(&on_api_base_url(utils::config::endpoints::jwt::IG_GENERATE).await)
        .build()
        .send_decode::<GenerateJwtResponse>()
        .await?;
    Ok((response.jwt, response.expires_at))
}

pub async fn generate_centrifugo_jwt() -> Result<(String, NaiveDateTime)> {
    let response = Request::get(&on_api_base_url(IG_GENERATE_CENTRIFUGO).await)
        .add_jwt()
        .await
        .build()
        .send_decode::<GenerateJwtResponse>()
        .await?;
    Ok((response.jwt, response.expires_at))
}
