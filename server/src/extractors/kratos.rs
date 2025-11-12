use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct KratosUserData {
    pub identity: KratosIdentity,
}

#[derive(Deserialize, Debug, Clone)]
struct KratosIdentity {
    pub traits: KratosTraits,
}

#[derive(Deserialize, Debug, Clone)]
struct KratosTraits {
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
            Ok(user) => Outcome::Success(Self {
                email: user.identity.traits.email,
            }),
            Err(e) => {
                eprintln!("Kratos cookie verification error: {}", e);
                Outcome::Error((Status::Unauthorized, "Cookie verification failed".to_string()))
            }
        }
    }
}

async fn verify_kratos_cookie(cookie_value: &str) -> Result<KratosUserData, String> {
    let url = format!(
        "{}/sessions/whoami",
        std::env::var("KRATOS_PUBLIC_URL").unwrap()
    );

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("Cookie", format!("ory_kratos_session={}", cookie_value))
        .send()
        .await
        .map_err(|e| format!("Failed to call Kratos: {e}"))?;

    if !res.status().is_success() {
        return Err(format!("Kratos verification failed: {}", res.status()));
    }

    res.json::<KratosUserData>()
        .await
        .map_err(|e| format!("Failed to parse Kratos response: {e}"))
}
