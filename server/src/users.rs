use axum::{
    http::HeaderMap, response::{IntoResponse, Response}, Json, 
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use anyhow::{anyhow, Context};

use crate::{db, schema::*, verify_jwt, verify_kratos_cookie, AppError};
use utils::requests::{GetUserRequest, GetUserResponse, ListUsersRequest, ListUsersResponse, SetupUserRequest, SetupUserResponse, UserInfo};

#[cfg(debug_assertions)]
pub async fn debug_user(Json(body): Json<UserInfo>) -> Result<Response, AppError> {
    tracing::info!("Debug user creation: {:?}", body);
    let db = db().await;
    tracing::info!("Database connection established");

    let user_model = users::ActiveModel {
        email: Set(format!("{}-{}", body.email, sea_orm::prelude::Uuid::new_v4())),
        username: Set(body.username),
        nickname: Set(body.nickname),
        ..Default::default()
    };
    let user = user_model
        .insert(db)
        .await
        .context("Failed to insert new user into database")?;

    let jwt = crate::jwt::generate(user.uuid)
        .await
        .context("Failed to generate JWT")?;

    let response = utils::requests::GenerateJwtResponse(jwt);
    Ok(Json(response).into_response())
}

pub async fn get_me(headers: HeaderMap) -> Result<Response, AppError> {
    let user_model = verify_jwt(&headers).await?;

    let user = UserInfo {
        uuid: user_model.uuid,
        email: user_model.email,
        username: user_model.username,
        nickname: user_model.nickname,
    };

    let response = GetUserResponse(user);
    Ok(Json(response).into_response())
}

pub async fn get_user(headers: HeaderMap, Json(body): Json<GetUserRequest>) -> Result<Response, AppError> {
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
        email: user_model.email,
        username: user_model.username,
        nickname: user_model.nickname,
    };

    let response = GetUserResponse(user);
    Ok(Json(response).into_response())
}

async fn username_taken(
    username: &str,
    db: &'static DatabaseConnection,
) -> Result<bool, AppError> {
    let user: Option<users::Model> = Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .context("Failed to query user from database")?;

    Ok(user.is_some())
}

pub async fn setup_user(headers: HeaderMap, Json(body): Json<SetupUserRequest>) -> Result<Response, AppError> {
    let email = verify_kratos_cookie(&headers)
        .await
        .context("Failed to verify Kratos cookie")?
        .identity.traits.email;
    let db = db().await;

    let _: users::Model = Users::find()
        .filter(users::Column::Email.eq(&email))
        .one(db)
        .await
        .context("Failed to query user from database")?
        .ok_or_else(|| anyhow!("User not found"))?;

    if body.username.len() < 3 || body.username.len() > 20 {
        return Err(anyhow!("Username must be between 3 and 20 characters").into());
    }
    if body.nickname.len() < 3 || body.nickname.len() > 30 {
        return Err(anyhow!("Nickname must be between 3 and 30 characters").into());
    }
    if !body.username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.' )
    {
        return Err(anyhow!("Username can only contain alphanumeric characterl, underscores, dots, and hyphens").into());
    }
    if username_taken(&body.username, db).await? {
        return Err(anyhow!("Username is already taken").into());
    }

    let new_user = users::ActiveModel {
        email: Set(email),
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

pub async fn list_users(headers: HeaderMap, Json(body): Json<ListUsersRequest>) -> Result<Response, AppError> {
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
            email: u.email,
            username: u.username,
            nickname: u.nickname,
        })
        .collect();

    let response = ListUsersResponse(users);
    Ok(Json(response).into_response())
}
