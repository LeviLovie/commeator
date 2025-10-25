use anyhow::{anyhow, Context};
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};

use crate::{db, schema::*, verify_jwt, verify_kratos_cookie, AppError};
use utils::{
    data::UserInfo,
    requests::{
        ChatUsersRequest, CheckUserResponse, GetUserRequest, GetUserResponse, GetUsernameRequest,
        ListUsersRequest, ListUsersResponse, SetupUserRequest, SetupUserResponse,
    },
};

#[cfg(debug_assertions)]
pub async fn debug_user(Json(body): Json<UserInfo>) -> Result<Response, AppError> {
    let db = db().await;

    let user_model = users::ActiveModel {
        username: Set(body.username),
        email_hash: Set(body.email_hash),
        nickname: Set(body.nickname),
        ..Default::default()
    };
    let user = user_model
        .insert(db)
        .await
        .context("Failed to insert new user into database")?;

    let (jwt, expires_at) = crate::jwt::generate(user.uuid)
        .await
        .context("Failed to generate JWT")?;

    let response = utils::requests::GenerateJwtResponse { jwt, expires_at };
    Ok(Json(response).into_response())
}

pub async fn check_user(headers: HeaderMap) -> Result<Response, AppError> {
    let email = verify_kratos_cookie(&headers).await?.identity.traits.email;
    let db = db().await;

    let user_model = Users::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .context("Failed to query user from database")?;

    let response = CheckUserResponse(user_model.is_some());
    Ok(Json(response).into_response())
}

pub async fn get_me(headers: HeaderMap) -> Result<Response, AppError> {
    let user_model = verify_jwt(&headers).await?;

    let user = UserInfo {
        uuid: user_model.uuid,
        email_hash: user_model.email_hash,
        username: user_model.username,
        nickname: user_model.nickname,
    };

    let response = GetUserResponse(user);
    Ok(Json(response).into_response())
}

pub async fn get_user(
    headers: HeaderMap,
    Json(body): Json<GetUserRequest>,
) -> Result<Response, AppError> {
    let _ = verify_jwt(&headers).await?;
    let db = db().await;

    let user_model: users::Model = Users::find()
        .filter(users::Column::Uuid.eq(body.0))
        .one(db)
        .await
        .context("Failed to query user from database")?
        .ok_or_else(|| anyhow!("User not found"))?;

    let user = UserInfo {
        uuid: user_model.uuid,
        email_hash: user_model.email_hash,
        username: user_model.username,
        nickname: user_model.nickname,
    };

    let response = GetUserResponse(user);
    Ok(Json(response).into_response())
}

pub async fn get_username(
    headers: HeaderMap,
    Json(body): Json<GetUsernameRequest>,
) -> Result<Response, AppError> {
    let _ = verify_jwt(&headers).await?;
    let db = db().await;

    let user_model: users::Model = Users::find()
        .filter(users::Column::Username.eq(body.0))
        .one(db)
        .await
        .context("Failed to query user from database")?
        .ok_or_else(|| anyhow!("User not found"))?;

    let user = UserInfo {
        uuid: user_model.uuid,
        email_hash: user_model.email_hash,
        username: user_model.username,
        nickname: user_model.nickname,
    };

    let response = GetUserResponse(user);
    Ok(Json(response).into_response())
}

async fn username_taken(username: &str, db: &'static DatabaseConnection) -> Result<bool, AppError> {
    let user: Option<users::Model> = Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .context("Failed to query user from database")?;

    Ok(user.is_some())
}

pub async fn setup_user(
    headers: HeaderMap,
    Json(body): Json<SetupUserRequest>,
) -> Result<Response, AppError> {
    let email = verify_kratos_cookie(&headers)
        .await
        .context("Failed to verify Kratos cookie")?
        .identity
        .traits
        .email;
    let db = db().await;

    let user_model: Option<users::Model> = Users::find()
        .filter(users::Column::Email.eq(&email))
        .one(db)
        .await
        .context("Failed to query user from database")?;

    if user_model.is_some() {
        return Err(anyhow!("User with this email already exists").into());
    }

    if body.username.len() < 3 || body.username.len() > 20 {
        return Err(anyhow!("Username must be between 3 and 20 characters").into());
    }
    if body.nickname.len() < 3 || body.nickname.len() > 30 {
        return Err(anyhow!("Nickname must be between 3 and 30 characters").into());
    }
    if !body
        .username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
    {
        return Err(anyhow!(
            "Username can only contain alphanumeric characterl, underscores, dots, and hyphens"
        )
        .into());
    }
    if username_taken(&body.username, db).await? {
        return Err(anyhow!("Username is already taken").into());
    }

    let email_hash = format!("{:x}", md5::compute(&email));

    let new_user = users::ActiveModel {
        email: Set(email),
        email_hash: Set(email_hash),
        username: Set(body.username),
        nickname: Set(body.nickname),
        ..Default::default()
    };
    new_user
        .insert(db)
        .await
        .context("Failed to insert new user into database")?;

    let response = SetupUserResponse {};
    Ok(Json(response).into_response())
}

pub async fn list_users(
    headers: HeaderMap,
    Json(body): Json<ListUsersRequest>,
) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;
    let db = db().await;

    let user_models: Vec<users::Model> = if body.exclude_self {
        Users::find()
            .filter(users::Column::Uuid.ne(user.uuid))
            .all(db)
            .await
            .context("Failed to query users from database")?
    } else {
        Users::find()
            .all(db)
            .await
            .context("Failed to query users from database")?
    };

    let users: Vec<UserInfo> = user_models
        .into_iter()
        .map(|u| UserInfo {
            uuid: u.uuid,
            email_hash: u.email_hash,
            username: u.username,
            nickname: u.nickname,
        })
        .collect();

    let response = ListUsersResponse(users);
    Ok(Json(response).into_response())
}

pub async fn chat_users(
    headers: HeaderMap,
    Json(body): Json<ChatUsersRequest>,
) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;
    let db = db().await;

    let chat_members: Vec<chat_members::Model> = ChatMembers::find()
        .filter(chat_members::Column::ChatUuid.eq(body.0))
        .all(db)
        .await
        .context("Failed to query chat members from database")?;

    if !chat_members.iter().any(|cm| cm.user_uuid == user.uuid) {
        return Err(anyhow!("User is not a member of this chat").into());
    }

    let user_uuids: Vec<sea_orm::prelude::Uuid> =
        chat_members.into_iter().map(|cm| cm.user_uuid).collect();

    let user_models: Vec<users::Model> = Users::find()
        .filter(users::Column::Uuid.is_in(user_uuids))
        .all(db)
        .await
        .context("Failed to query users from database")?;

    let users: Vec<UserInfo> = user_models
        .into_iter()
        .map(|u| UserInfo {
            uuid: u.uuid,
            email_hash: u.email_hash,
            username: u.username,
            nickname: u.nickname,
        })
        .collect();

    let response = ListUsersResponse(users);
    Ok(Json(response).into_response())
}
