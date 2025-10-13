use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
mod server_utils {
    pub use dioxus::prelude::dioxus_fullstack::HeaderMap;
    pub use sea_orm::DatabaseConnection;

    pub use crate::backend::server_utils::*;
}
#[cfg(feature = "server")]
use server_utils::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub nickname: String,
}

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

#[post("/api/user/me")]
pub async fn get_my_user(jwt: String) -> Result<UserInfo, ServerFnError> {
    let user_model = verify_jwt(&jwt).await?;

    let user = UserInfo {
        id: user_model.id,
        email: user_model.email,
        username: user_model.username,
        nickname: user_model.nickname,
    };

    Ok(user)
}

#[post("/api/user/get")]
pub async fn get_user(jwt: String, username: String) -> Result<UserInfo, ServerFnError> {
    let _ = verify_jwt(&jwt).await?;

    let db = db().await;

    let user_model: Option<users::Model> = Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let user: UserInfo = match user_model {
        Some(u) => UserInfo {
            id: u.id,
            email: u.email,
            username: u.username,
            nickname: u.nickname,
        },
        None => {
            return Err(ServerFnError::new("User not found".to_string()));
        }
    };

    Ok(user)
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

    if username.len() < 3 || username.len() > 20 {
        return Err(ServerFnError::new(
            "Username must be between 3 and 20 characters".to_string(),
        ));
    }
    if nickname.len() < 3 || nickname.len() > 30 {
        return Err(ServerFnError::new(
            "Nickname must be between 3 and 30 characters".to_string(),
        ));
    }
    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(ServerFnError::new(
            "Username can only contain alphanumeric characters, underscores, and hyphens"
                .to_string(),
        ));
    }
    if username_taken(&username, db).await? {
        return Err(ServerFnError::new("Username is already taken".to_string()));
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

#[get("/api/user/by_username")]
pub async fn user_by_username(jwt: String) -> Result<Option<i32>, ServerFnError> {
    let _ = verify_jwt(&jwt).await?;
    let db = db().await;

    let user: Option<users::Model> = Users::find()
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    match user {
        Some(u) => Ok(Some(u.id)),
        None => Ok(None),
    }
}

#[cfg(feature = "server")]
async fn username_taken(
    username: &str,
    db: &'static DatabaseConnection,
) -> Result<bool, ServerFnError> {
    let user: Option<users::Model> = Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(user.is_some())
}

#[post("/api/user/list")]
pub async fn list_users(jwt: String) -> Result<Vec<UserInfo>, ServerFnError> {
    let _ = verify_jwt(&jwt).await?;
    let db = db().await;

    let user_models: Vec<users::Model> = Users::find()
        .all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let users: Vec<UserInfo> = user_models
        .into_iter()
        .map(|u| UserInfo {
            id: u.id,
            email: u.email,
            username: u.username,
            nickname: u.nickname,
        })
        .collect();

    Ok(users)
}
