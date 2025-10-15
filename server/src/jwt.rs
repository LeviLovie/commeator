use axum::{
    http::HeaderMap, response::{IntoResponse, Response}, Json, 
};
use sea_orm::{prelude::Uuid, ColumnTrait, EntityTrait, QueryFilter};
use anyhow::{anyhow, Context};

use crate::{db, verify_kratos_cookie, AppError, schema::*};
use utils::{config::server::jwt_secret, requests::{GenerateJwtResponse, VerifyJwtResponse}};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
}

pub async fn verify_jwt(headers: &HeaderMap) -> anyhow::Result<users::Model> {
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

pub async fn generate(uuid: Uuid) -> anyhow::Result<String> {
    let expiration = chrono::Utc::now() + chrono::Duration::hours(24);
    let claims = Claims {
        sub: uuid,
        exp: expiration.timestamp(),
    };

    let header = jsonwebtoken::Header::default();
    jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(&jwt_secret()),
    )
        .context("Failed to encode JWT")
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

    let token = generate(user.uuid).await?;

    let response = GenerateJwtResponse(token);
    Ok(Json(response).into_response())
}

pub async fn endpoint_verify(headers: HeaderMap) -> Result<Response, AppError> {
    let correct = verify_jwt(&headers)
        .await.is_ok();
    
    let response = VerifyJwtResponse(correct);
    Ok(Json(response).into_response())
}
