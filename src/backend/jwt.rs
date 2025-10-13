use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

#[cfg(feature = "server")]
mod server_utils {
    pub use dioxus::prelude::dioxus_fullstack::HeaderMap;

    pub use crate::backend::server_utils::*;
    pub use crate::config::jwt_secret;
}
#[cfg(feature = "server")]
use server_utils::*;

#[post("/api/jwt/generate", headers: HeaderMap)]
pub async fn generate_jwt() -> Result<String, ServerFnError> {
    let email = verify_kratos_cookie(&headers)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .identity
        .traits
        .email;

    let db = db().await;
    let user = Users::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("User not found".to_string()))?;

    let expiration = chrono::Utc::now() + chrono::Duration::hours(24);
    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: expiration.timestamp() as usize,
    };

    let header = jsonwebtoken::Header::default();
    let token = jsonwebtoken::encode(
        &header,
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(&jwt_secret()),
    )
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    info!("Generated JWT for user {}", user.email);
    Ok(token)
}

#[cfg(feature = "server")]
pub async fn verify_jwt(token: &str) -> Result<users::Model, ServerFnError> {
    let claims = jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(&jwt_secret()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| ServerFnError::new("Invalid token".to_string()))?
    .claims;

    let db = db().await;
    let user = Users::find()
        .filter(users::Column::Id.eq(claims.sub.parse::<i32>().unwrap()))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("User not found".to_string()))?;

    Ok(user)
}

#[post("/api/jwt/verify")]
pub async fn verify_jwt_endpoint(jwt: String) -> Result<bool, ServerFnError> {
    Ok(verify_jwt(&jwt).await.is_ok())
}
