use anyhow::{Context, Result};
use dioxus::prelude::*;
use dioxus_fullstack::HeaderMap;

use crate::auth::{get_user_from_cookie, KratosUserData};

pub async fn verify_kratos_cookie(headers: &HeaderMap) -> Result<KratosUserData> {
    let cookie = headers
        .get("Cookie")
        .context("No Cookie header found")?
        .to_str()
        .context("Failed to convert Cookie header to str")?;

    get_user_from_cookie(cookie)
        .await
        .context("Failed to verify Kratos cookie")
}
