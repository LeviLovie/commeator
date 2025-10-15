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

#[cfg(feature = "client")]
pub mod client {
    use dioxus::prelude::*;
    use gloo_net::http::Request;
    
    use super::*;

    async fn try_get_user() -> Result<KratosUserData> {
        match Request::get(URL_WHOAMI)
            .credentials(web_sys::RequestCredentials::Include)
            .send()
            .await
            .context("Failed to send request to Kratos")?
        {
            r if r.ok() => r
                .json::<KratosUserData>()
                .await
                .context("Failed to parse user info from Kratos"),
            r => Err(anyhow::anyhow!("Received non-OK response: {}", r.status())),
        }
    }

    pub async fn get_user() -> Option<KratosUserData> {
        match try_get_user().await {
            Ok(user) => Some(user),
            Err(e) => {
                error!("Error fetching user info: {}", e);
                navigator().replace(URL_LOGIN);
                None
            }
        }
    }
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

#[macro_export]
macro_rules! verify_user_jwt {
    () => {{
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
            async move {
                if let Some(token) = jwt_clone {
                    if token == "NO_JWT" {
                        false
                    } else {
                        $crate::backend::jwt::verify_jwt_endpoint(token)
                            .await
                            .unwrap_or(false)
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
    }};
}
