use std::sync::{LazyLock, Mutex};

use dioxus::prelude::*;

use crate::{backend::jwt::generate_jwt, components::logout};

static JWT: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

pub async fn request_jwt() -> Result<(), ServerFnError> {
    let jwt = generate_jwt().await?;

    JWT.lock().unwrap().replace(jwt);

    Ok(())
}

pub async fn jwt() -> String {
    if JWT.lock().unwrap().is_none() {
        if let Err(e) = request_jwt().await {
            if e.to_string().contains("User not found") {
                logout().await;
            }
            error!("Failed to request JWT: {}", e);
            return "NO_JWT".to_string();
        }
    }

    match JWT.lock().unwrap().as_ref() {
        Some(token) => token.clone(),
        None => "NO_JWT".to_string(),
    }
}
