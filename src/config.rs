pub const URL_WHOAMI: &str = "http://localhost:4433/sessions/whoami";
pub const URL_LOGIN: &str = "http://localhost:4433/self-service/login/browser";
pub const URL_LOGOUT: &str = "http://localhost:4433/self-service/logout/browser";

pub fn url_login_flow(flow_id: &str) -> String {
    format!(
        "http://localhost:4433/self-service/login/flows?id={}",
        flow_id
    )
}

#[cfg(feature = "server")]
pub mod server_utils {
    pub const DATABASE_URL: &str = "postgresql://messenger@localhost/messenger";
}

#[cfg(feature = "server")]
pub fn jwt_secret() -> Vec<u8> {
    dotenv::dotenv().ok();

    match std::env::var("JWT_SECRET") {
        Ok(secret) => secret.into_bytes(),
        Err(_) => panic!("JWT_SECRET not present"),
    }
}
