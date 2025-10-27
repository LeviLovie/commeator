use std::sync::{LazyLock, Mutex};
use anyhow::{Context};
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::prelude::Uuid;

use crate::{jwt, verify_jwt, AppError};
use utils::{requests::{
        NativesAuthenticateRequest, NativesAuthenticateResponse, NativesIsAuthenticatedRequest, NativesIsAuthenticatedResponse
    }};

static NATIVE_KEYS: LazyLock<Mutex<Vec<(Uuid, String)>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub async fn authenticate(
    headers: HeaderMap,
    Json(body): Json<NativesAuthenticateRequest>,
) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;

    let mut keys = NATIVE_KEYS.lock().unwrap();
    keys.push((user.uuid, body.0.to_string()));

    let response = NativesAuthenticateResponse {};
    Ok(Json(response).into_response())
}

pub async fn is_authenticated(
    Json(body): Json<NativesIsAuthenticatedRequest>,
) -> Result<Response, AppError> {
    let keys = NATIVE_KEYS.lock().unwrap().clone();

    match keys.iter().find(|(_, key)| key == &body.0.to_string()) {
        None => {
            let response = NativesIsAuthenticatedResponse(None);
            Ok(Json(response).into_response())
        }
        Some((uuid, _)) => {
            let jwt = jwt::generate(*uuid).await.context("Failed to generate JWT")?;
            let response = NativesIsAuthenticatedResponse(Some(jwt.0));
            Ok(Json(response).into_response())
        }
    }
}

