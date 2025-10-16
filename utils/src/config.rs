pub fn env_value(key: &str) -> String {
    if let Err(e) = dotenvy::from_filename(".env") {
        panic!("Failed to read .env: {}", e);
    }

    match std::env::var(key) {
        Ok(value) => value,
        Err(_) => panic!("{} not present", key),
    }
}

#[cfg(feature = "server")]
pub mod server {
    use super::env_value;

    pub fn db_url() -> String {
        env_value("DATABASE_URL").trim_end_matches('/').to_string()
    }

    pub fn centrifugo_url() -> String {
        env_value("BASE_URL_CENTRIFUGO")
            .trim_end_matches('/')
            .to_string()
    }

    pub fn centrifugo_key() -> String {
        env_value("CENTRIFUGO_KEY").to_string()
    }

    pub fn jwt_secret() -> Vec<u8> {
        env_value("JWT_SECRET").as_bytes().to_vec()
    }

    pub fn centrifugo_jwt_secret() -> Vec<u8> {
        env_value("CENTRIFUGO_JWT_SECRET").as_bytes().to_vec()
    }
}

pub mod endpoints {
    pub mod auth {
        pub const URI_WHOAMI: &str = "/sessions/whoami";
        pub const URI_LOGIN: &str = "/self-service/login/browser";
        pub const URI_LOGOUT: &str = "/self-service/logout/browser";

        pub fn url_login_flow(flow_id: &str) -> String {
            let url = format!(
                "{}/self-service/login/flows?id={}",
                super::super::auth_base_url(),
                flow_id
            );
            url
        }
    }

    pub mod jwt {
        pub const IG_GENERATE: &str = "/jwt/generate";
        pub const IG_VERIFY: &str = "/jwt/verify";
        pub const IG_GENERATE_CENTRIFUGO: &str = "/jwt/centrifugo";
    }

    pub mod chats {
        pub const IG_LIST: &str = "/chats/list";
        pub const IP_GET: &str = "/chats/get";
        pub const IP_VERIFY_PRIVATE: &str = "/chats/verify_private";
    }

    pub mod messages {
        pub const IP_LIST: &str = "/messages/list";
        pub const IP_SEND: &str = "/messages/send";
    }

    pub mod users {
        pub const IG_CHECK: &str = "/users/check";
        pub const IG_ME: &str = "/users/me";
        pub const IP_GET: &str = "/users/get";
        pub const IP_SETUP: &str = "/users/setup";
        pub const IP_LIST: &str = "/users/list";
    }
}

#[cfg(feature = "client")]
pub fn auth_base_url() -> String {
    let url: String = env!("BASE_URL_AUTH").into();
    url.trim_end_matches('/').into()
}

#[cfg(not(feature = "client"))]
pub fn auth_base_url() -> String {
    env_value("BASE_URL_AUTH").trim_end_matches('/').to_string()
}

#[cfg(feature = "client")]
pub fn api_base_url() -> String {
    let url: String = env!("BASE_URL_API").into();
    url.trim_end_matches('/').into()
}

#[cfg(not(feature = "client"))]
pub fn api_base_url() -> String {
    env_value("BASE_URL_API").trim_end_matches('/').to_string()
}

pub fn on_api_base_url(uri: &'static str) -> String {
    let uri = uri.trim_start_matches('/');
    let url = format!("{}/{}", api_base_url(), uri);
    url
}

pub fn on_auth_base_url(uri: &'static str) -> String {
    let uri = uri.trim_start_matches('/');
    let url = format!("{}/{}", auth_base_url(), uri);
    url
}
