use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
}

#[derive(Debug, Clone)]
pub enum JwtStatus {
    Valid(Claims),
    Expired,
    Invalid,
}

pub fn verify(jwt: String) -> JwtStatus {
    match jsonwebtoken::decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(token) => JwtStatus::Valid(token.claims),
        Err(e) => match e.kind() {
            ErrorKind::ExpiredSignature => JwtStatus::Expired,
            _ => JwtStatus::Invalid,
        },
    }
}

pub fn create(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_ref()),
    )
}

// TODO: Add tests
