use once_cell::sync::Lazy;

pub const AUTH_WHOAMI: &str = "/sessions/whoami";
pub const AUTH_LOGIN: &str = "/self-service/login/browser";
pub const AUTH_LOGIN_FLOW: &str = "/self-service/login/flows";
pub const AUTH_LOGOUT: &str = "/self-service/logout/browser";
pub const AUTH_NATIVE_REDIRECT: &str = "/a/redirect";

#[cfg(debug_assertions)]
#[cfg(target_arch = "wasm32")]
const ENV_FILE: &str = include_str!("../.env.web.local");
#[cfg(debug_assertions)]
#[cfg(not(target_arch = "wasm32"))]
const ENV_FILE: &str = include_str!("../.env.native.local");
#[cfg(not(debug_assertions))]
#[cfg(target_arch = "wasm32")]
const ENV_FILE: &str = include_str!("../.env.web");
#[cfg(not(debug_assertions))]
#[cfg(not(target_arch = "wasm32"))]
const ENV_FILE: &str = include_str!("../.env.native");

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut api = None;
    let mut ws = None;
    let mut auth = None;

    for item in dotenvy::from_read_iter(ENV_FILE.as_bytes()) {
        let (key, val) = item.unwrap();

        match key.as_str() {
            "URL_API" => api = Some(val),
            "URL_WS" => ws = Some(val),
            "URL_AUTH" => auth = Some(val),
            _ => {}
        }
    }

    Config {
        url_api: clean_up_url(api.expect("URN_API missing")),
        url_ws: clean_up_url(ws.expect("URN_WS missing")),
        url_auth: clean_up_url(auth.expect("URN_AUTH missing")),
    }
});

fn clean_up_url(url: String) -> String {
    url.trim_end_matches('/').to_string()
}

#[derive(Debug, Clone)]
pub struct Config {
    pub url_api: String,
    pub url_ws: String,
    pub url_auth: String,
}
