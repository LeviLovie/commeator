use once_cell::sync::Lazy;

#[cfg(debug_assertions)]
const ENV_FILE: &str = include_str!("../.env.local");
#[cfg(not(debug_assertions))]
const ENV_FILE: &str = include_str!("../.env");

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut db_url = None;
    let mut web_url = None;
    let mut jwt_secret = None;
    let mut auth = None;

    for item in dotenvy::from_read_iter(ENV_FILE.as_bytes()) {
        let (key, val) = item.unwrap();

        match key.as_str() {
            "DATABASE_URL" => db_url = Some(val),
            "JWT_SECRET" => jwt_secret = Some(val),
            "URL_AUTH" => auth = Some(val),
            "URL_WEB" => web_url = Some(val),
            _ => {}
        }
    }

    Config {
        db_url: db_url.expect("DATABASE_URL missing"),
        jwt_secret: jwt_secret.expect("JWT_SECRET missing"),
        url_auth: clean_up_url(auth.expect("URN_AUTH missing")),
        url_web: clean_up_url(web_url.expect("URL_WEB missing")),
    }
});

fn clean_up_url(url: String) -> String {
    url.trim_end_matches('/').to_string()
}

#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
    pub jwt_secret: String,
    pub url_auth: String,
    pub url_web: String,
}
