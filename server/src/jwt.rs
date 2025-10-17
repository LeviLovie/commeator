use anyhow::{anyhow, Result, Context};
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sea_orm::{prelude::Uuid, ColumnTrait, EntityTrait, QueryFilter};

use crate::{db, schema::*, verify_kratos_cookie, AppError};
use utils::{
    config::server::{centrifugo_jwt_secret, jwt_secret},
    requests::{GenerateJwtResponse, VerifyJwtResponse},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
}

pub async fn verify_jwt(headers: &HeaderMap) -> Result<users::Model> {
    let token = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| {
            if h.starts_with("Bearer ") {
                Some(h.trim_start_matches("Bearer ").to_string())
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow!("Missing or invalid Authorization header"))?;

    let claims = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(&jwt_secret()),
        &jsonwebtoken::Validation::default(),
    )
    .context("Failed to decode JWT")?
    .claims;

    let db = db().await;
    let user = Users::find()
        .filter(users::Column::Uuid.eq(claims.sub))
        .one(db)
        .await
        .context("Failed to query user from database")?
        .ok_or_else(|| anyhow!("User not found"))?;

    Ok(user)
}

pub async fn generate(uuid: Uuid) -> Result<(String, NaiveDateTime)> {
    let expiration = Utc::now() + Duration::hours(24);
    let claims = Claims {
        sub: uuid,
        exp: expiration.timestamp(),
    };

    let header = jsonwebtoken::Header::default();
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(&jwt_secret()),
    )
    .context("Failed to encode JWT")?;

    Ok((token, expiration.naive_utc()))
}

pub async fn endpoint_generate(headers: HeaderMap) -> Result<Response, AppError> {
    let email = verify_kratos_cookie(&headers)
        .await
        .context("Failed to verify Kratos cookie")?
        .identity
        .traits
        .email;

    let db = db().await;
    let user = Users::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .context("Failed to query user from database")?
        .ok_or_else(|| anyhow!("User not found"))?;

    let (jwt, expires_at) = generate(user.uuid).await?;

    let response = GenerateJwtResponse {
        jwt,
        expires_at,
    };
    Ok(Json(response).into_response())
}

pub async fn endpoint_verify(headers: HeaderMap) -> Result<Response, AppError> {
    let correct = verify_jwt(&headers).await.is_ok();

    let response = VerifyJwtResponse(correct);
    Ok(Json(response).into_response())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentrifugoClaims {
    pub sub: Uuid,
    pub exp: i64,
    pub channels: Vec<String>,
}

pub async fn generate_centrifugo_token(uuid: Uuid, channels: Vec<String>) -> Result<(String, NaiveDateTime)> {
    let expiration = Utc::now() + Duration::minutes(15);
    let claims = CentrifugoClaims {
        sub: uuid,
        exp: expiration.timestamp(),
        channels,
    };

    let header = jsonwebtoken::Header::default();
    let jwt = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(&centrifugo_jwt_secret()), // same secret, or a separate one
    )
    .context("Failed to encode Centrifugo JWT")?;

    Ok((jwt, expiration.naive_utc()))
}

pub async fn endpoint_generate_centrifugo(
    headers: HeaderMap,
) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await.context("Failed to verify JWT")?;

    let chats_member = chat_members::Entity::find()
        .filter(chat_members::Column::UserUuid.eq(user.uuid))
        .all(db().await)
        .await
        .context("Failed to query chat memberships from database")?;

    let channels = chats_member
        .into_iter()
        .map(|cm| format!("chat_{}", cm.chat_uuid))
        .collect();

    let (jwt, expires_at) = generate_centrifugo_token(user.uuid, channels).await?;

    let response = GenerateJwtResponse {
        jwt,
        expires_at,
    };
    Ok(Json(response).into_response())
}
