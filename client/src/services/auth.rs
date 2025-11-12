use dioxus::prelude::*;
use chrono::{Utc, NaiveDateTime};
use anyhow::{Context, Result};

pub struct AuthService {
    pub jwt: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
}

impl AuthService {
    pub fn load_or_generate(storage: &StorageService) -> Result<Self> {
        let mut auth_service = AuthService {
            jwt: None,
            expires_at: None,
        };

        auth_service.load_from_storage(storage).context("Failed to load JWT from storage")?;

        if auth_service.jwt.is_none() || auth_service.is_expired() {
            auth_service.regenerate().context("Failed to regenerate JWT")?;
        }

        Ok(auth_service)
    }

    pub async fn load_from_storage(&mut self, storage: &StorageService) {
        if let Some(token) = storage.load_jwt().await {
            self.jwt = Some(token);
        }
    }

    pub async fn regenerate(&self) -> Result<()> {
        let new_token = unimplemented!("Regenerate token logic goes here");
        self.jwt = Some(new_token.jwt);
        self.expires_at = Some(new_token.expires_at);
        Ok(())
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at().map_or(true, |t| t <= Utc::now().naive_utc())
    }
}

#[macro_export]
macro_rules! verify_user {
    () => {
        let user = use_resource(|| async { $crate::backend::get_kratos_user().await });
        if user().is_none() || user().as_ref().unwrap().is_none() {
            return rsx! { $crate::components::Spinner {} };
        }
    };
}
