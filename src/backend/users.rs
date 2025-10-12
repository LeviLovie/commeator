use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus_fullstack::HeaderMap;

#[cfg(feature = "server")]
use super::server_utils::*;

#[post("/api/user/check", headers: HeaderMap)]
pub async fn check_user() -> Result<bool, ServerFnError> {
    let user = verify_kratos_cookie(&headers)
        .await
        .context("Failed to verify Kratos cookie")?;
    let email = user.identity.traits.email.clone();

    let db = db().await;

    let user: Option<users::Model> = Users::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(user.is_some())
}

#[post("/api/user/setup", headers: HeaderMap)]
pub async fn setup_user(username: String, nickname: String) -> Result<(), ServerFnError> {
    let user = verify_kratos_cookie(&headers)
        .await
        .context("Failed to verify Kratos cookie")?;
    let email = user.identity.traits.email.clone();

    let db = db().await;

    let user: Option<users::Model> = Users::find()
        .filter(users::Column::Email.eq(&email))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    if user.is_some() {
        return Err(ServerFnError::new("User already exists".to_string()));
    }

    let new_user = users::ActiveModel {
        email: Set(email),
        username: Set(username),
        nickname: Set(nickname),
        ..Default::default()
    };
    new_user
        .insert(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
