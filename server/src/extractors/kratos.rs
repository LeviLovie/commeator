use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::Deserialize;

use crate::config::CONFIG;

const AUTH_WHOAMI: &str = "/sessions/whoami";

#[derive(Deserialize, Debug, Clone)]
pub struct KratosUserData {
    pub identity: Identity,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Identity {
    pub traits: Traits,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Traits {
    pub email: String,
}

pub struct Kratos {
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Kratos {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let session_cookie = cookies
            .get("ory_kratos_session")
            .map(|c| c.value().to_string());

        let cookie_value = match session_cookie {
            Some(c) => c,
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    "No Kratos session cookie found".into(),
                ));
            }
        };

        match verify_kratos_cookie(&cookie_value).await {
            Ok(Some(email)) => Outcome::Success(Self { email }),
            Ok(None) => {
                Outcome::Error((Status::Unauthorized, "Invalid Kratos session".to_string()))
            }
            Err(e) => {
                eprintln!("Kratos cookie verification error: {}", e);
                Outcome::Error((
                    Status::Unauthorized,
                    "Cookie verification failed".to_string(),
                ))
            }
        }
    }
}

async fn verify_kratos_cookie(cookie_value: &str) -> Result<Option<String>, String> {
    let res = reqwest::Client::new()
        .get(format!("{}{}", CONFIG.url_auth, AUTH_WHOAMI))
        .header("Cookie", format!("ory_kratos_session={}", cookie_value))
        .send()
        .await
        .map_err(|e| format!("Failed to call Kratos: {e}"))?;

    if !res.status().is_success() {
        return Ok(None);
    }

    let user_data = res
        .json::<KratosUserData>()
        .await
        .map_err(|e| format!("Failed to parse Kratos response: {e}"));

    user_data.map(|data| Some(data.identity.traits.email))
}
