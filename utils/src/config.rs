use async_once_cell::OnceCell;

#[cfg(debug_assertions)]
pub fn env_value(key: &str) -> String {
    if let Err(e) = dotenvy::from_filename(".env") {
        panic!("Failed to read .env: {}", e);
    }

    match std::env::var(key) {
        Ok(value) => value,
        Err(_) => panic!("{} not present", key),
    }
}

#[cfg(not(debug_assertions))]
pub fn env_value(key: &str) -> String {
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

        #[cfg(feature = "client")]
        pub async fn url_login_flow(flow_id: &str) -> String {
            let url = format!(
                "{}/self-service/login/flows?id={}",
                super::super::auth_base_url().await,
                flow_id
            );
            url
        }

        #[cfg(not(feature = "client"))]
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

    pub mod groups {
        pub const IP_NEW: &str = "/groups/new";
    }

    pub mod messages {
        pub const IP_LIST: &str = "/messages/list";
        pub const IP_SEND: &str = "/messages/send";
        pub const IP_DELETE: &str = "/messages/delete";
        pub const IP_EDIT: &str = "/messages/edit";
    }

    pub mod users {
        pub const IG_CHECK: &str = "/users/check";
        pub const IG_ME: &str = "/users/me";
        pub const IP_GET: &str = "/users/get";
        pub const IP_NAME: &str = "/users/name";
        pub const IP_SETUP: &str = "/users/setup";
        pub const IP_LIST: &str = "/users/list";
        pub const IP_CHAT: &str = "/users/chat";
    }
}

#[allow(dead_code)]
static WEB_CONFIG: OnceCell<serde_json::Value> = OnceCell::new();

#[cfg(feature = "client")]
#[allow(dead_code)]
pub async fn web_config() -> serde_json::Value {
    WEB_CONFIG
        .get_or_init(async {
            tracing::warn!("Fetching web config from /endpoints");
            gloo_net::http::Request::get("/endpoints")
                .header("Cache-Control", "no-cache")
                .send()
                .await
                .expect("Failed to fetch web config")
                .json()
                .await
                .expect("Failed to parse web config")
        })
        .await
        .clone()
}

#[cfg(feature = "client")]
pub async fn auth_base_url() -> String {
    #[cfg(not(debug_assertions))]
    {
        let config = web_config().await;
        config["auth"]
            .as_str()
            .expect("auth not found in web config")
            .trim_end_matches('/')
            .to_string()
    }
    #[cfg(debug_assertions)]
    {
        env!("BASE_URL_AUTH").trim_end_matches('/').to_string()
    }
}

#[cfg(not(feature = "client"))]
pub fn auth_base_url() -> String {
    env_value("BASE_URL_AUTH").trim_end_matches('/').to_string()
}

#[cfg(feature = "client")]
pub async fn api_base_url() -> String {
    #[cfg(not(debug_assertions))]
    {
        let config = web_config().await;
        config["api"]
            .as_str()
            .expect("api not found in web config")
            .trim_end_matches('/')
            .to_string()
    }
    #[cfg(debug_assertions)]
    {
        env!("BASE_URL_API").trim_end_matches('/').to_string()
    }
}

#[cfg(feature = "client")]
pub async fn wss_base_url() -> String {
    #[cfg(not(debug_assertions))]
    {
        let config = web_config().await;
        config["wss"]
            .as_str()
            .expect("wss not found in web config")
            .trim_end_matches('/')
            .to_string()
    }
    #[cfg(debug_assertions)]
    {
        env!("BASE_URL_WSS").trim_end_matches('/').to_string()
    }
}

#[cfg(not(feature = "client"))]
pub fn api_base_url() -> String {
    env_value("BASE_URL_API").trim_end_matches('/').to_string()
}

#[cfg(feature = "client")]
pub async fn on_api_base_url(uri: &'static str) -> String {
    let uri = uri.trim_start_matches('/');
    let api_base_url = api_base_url().await;
    let url = format!("{}/{}", api_base_url, uri);
    url
}

#[cfg(not(feature = "client"))]
pub fn on_api_base_url(uri: &'static str) -> String {
    let uri = uri.trim_start_matches('/');
    let url = format!("{}/{}", api_base_url(), uri);
    url
}

#[cfg(feature = "client")]
pub async fn on_auth_base_url(uri: &'static str) -> String {
    let uri = uri.trim_start_matches('/');
    let auth_base_url = auth_base_url().await;
    let url = format!("{}/{}", auth_base_url, uri);
    url
}

#[cfg(not(feature = "client"))]
pub fn on_auth_base_url(uri: &'static str) -> String {
    let uri = uri.trim_start_matches('/');
    let url = format!("{}/{}", auth_base_url(), uri);
    url
}
