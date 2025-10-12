use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::server_utils::*;

#[server]
pub async fn check_user(email: String) -> Result<bool, ServerFnError> {
    let db = db().await;
    let user: Option<users::Model> = Users::find().filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(user.is_some())
}

#[server]
pub async fn setup_user(email: String, username: String, nickname: String) -> Result<(), ServerFnError> {
    let db = db().await;

    let user: Option<users::Model> = Users::find().filter(users::Column::Email.eq(&email))
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
    new_user.insert(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
