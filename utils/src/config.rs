#[cfg(feature = "server")]
pub mod server {
    pub fn db_url() -> String {
        dotenv::dotenv().ok();

        match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => panic!("DATABASE_URL not present"),
        }
    }

    pub fn jwt_secret() -> Vec<u8> {
        dotenv::dotenv().ok();

        match std::env::var("JWT_SECRET") {
            Ok(secret) => secret.into_bytes(),
            Err(_) => panic!("JWT_SECRET not present"),
        }
    }
}

pub mod endpoints {
    pub mod auth {
        pub const URI_WHOAMI: &str = "/sessions/whoami";
        pub const URI_LOGIN: &str = "/self-service/login/browser";
        pub const URI_LOGOUT: &str = "/self-service/logout/browser";

        pub fn url_login_flow(flow_id: &str) -> String {
            format!(
                "{}/self-service/login/flows?id={}",
                super::super::auth_base_url(),
                flow_id
            )
        }
    }

    pub mod jwt {
        pub const IG_GENERATE: &str = "/jwt/generate";
        pub const IG_VERIFY: &str = "/jwt/verify";
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
    dotenv::dotenv().ok();

    let url = match std::env::var("BASE_URL_AUTH") {
        Ok(url) => url,
        Err(_) => panic!("BASE_URL_AUTH not present"),
    };

    url.trim_end_matches('/').into()
}

#[cfg(feature = "client")]
pub fn api_base_url() -> String {
    let url: String = env!("BASE_URL_API").into();
    url.trim_end_matches('/').into()
}

#[cfg(not(feature = "client"))]
pub fn api_base_url() -> String {
    dotenv::dotenv().ok();

    let url = match std::env::var("BASE_URL_API") {
        Ok(url) => url,
        Err(_) => panic!("BASE_URL_AUTH not present"),
    };

    url.trim_end_matches('/').into()
}

pub fn on_api_base_url(uri: &'static str) -> String {
    let uri = uri.trim_start_matches('/');
    let url = format!("{}/{}", api_base_url(), uri);
    tracing::info!("API URL: {}", url);
    url
}

pub fn on_auth_base_url(uri: &'static str) -> String {
    tracing::info!("Auth URI: {}", uri);
    tracing::info!("Auth Base URL: {}", auth_base_url());
    let uri = uri.trim_start_matches('/');
    let url = format!("{}/{}", auth_base_url(), uri);
    tracing::info!("Auth URL: {}", url);
    url
}
