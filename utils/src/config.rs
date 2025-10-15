pub mod auth {
    pub const URL_WHOAMI: &str = "http://localhost:4433/sessions/whoami";
    pub const URL_LOGIN: &str = "http://localhost:4433/self-service/login/browser";
    pub const URL_LOGOUT: &str = "http://localhost:4433/self-service/logout/browser";

    pub fn url_login_flow(flow_id: &str) -> String {
        format!(
            "http://localhost:4433/self-service/login/flows?id={}",
            flow_id
        )
    }
}

pub const SERVER_URL: &str = "http://localhost:3000";

#[cfg(feature = "server")]
pub mod server {
    pub const DATABASE_URL: &str = "postgresql://messenger@localhost/messenger";

    pub fn jwt_secret() -> Vec<u8> {
        dotenv::dotenv().ok();

        match std::env::var("JWT_SECRET") {
            Ok(secret) => secret.into_bytes(),
            Err(_) => panic!("JWT_SECRET not present"),
        }
    }
}

pub mod endpoints {
    pub mod jwt {
        pub const G_GENERATE: &str = "/jwt/generate";
        pub const G_VERIFY: &str = "/jwt/verify";
    }

    pub mod chats {
        pub const G_LIST: &str = "/chats/list";
        pub const P_GET: &str = "/chats/get";
        pub const P_VERIFY_PRIVATE: &str = "/chats/verify_private";
    }

    pub mod messages {
        pub const P_LIST: &str = "/messages/list";
        pub const P_SEND: &str = "/messages/send";
    }

    pub mod users {
        pub const G_CHECK: &str = "/users/check";
        pub const G_ME: &str = "/users/me";
        pub const P_GET: &str = "/users/get";
        pub const P_SETUP: &str = "/users/setup";
        pub const P_LIST: &str = "/users/list";
    }
}

pub fn server_url(url: &'static str) -> String {
    format!("{}{}", SERVER_URL, url)
}
