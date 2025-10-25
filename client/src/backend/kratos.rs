use dioxus::prelude::*;

use utils::{
    auth::KratosUserData,
    config::{endpoints::auth::URI_LOGIN, on_auth_base_url},
};

#[cfg(target_arch = "wasm32")]
pub async fn try_get_kratos_user() -> Result<KratosUserData> {
    super::Request::get(&on_auth_base_url(utils::config::endpoints::auth::URI_WHOAMI).await)
        .build()
        .send_decode::<KratosUserData>()
        .await
}

#[cfg(target_arch = "wasm32")]
pub async fn get_kratos_user() -> Option<KratosUserData> {
    match try_get_kratos_user().await {
        Ok(user) => Some(user),
        Err(_) => {
            navigator().replace(on_auth_base_url(URI_LOGIN).await);
            None
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_kratos_user() -> Option<KratosUserData> {
    use crate::backend::get_jwt;

    if get_jwt().await.is_none()
        && let Err(e) = open::with(
            format!(
                "{}?return_to={}",
                on_auth_base_url(URI_LOGIN).await,
                utils::config::auth_return_to().await
            ),
            "/Applications/Google Chrome.app",
        )
    {
        error!("Failed to open browser for login: {}", e);
    }

    Some(KratosUserData {
        identity: utils::auth::Identity {
            traits: utils::auth::Traits {
                email: "".to_string(),
            },
        },
    })
}
