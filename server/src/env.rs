use once_cell::sync::Lazy;
use std::env;

static _DOTENV: Lazy<()> = Lazy::new(|| {
    dotenvy::dotenv().ok();
});

pub fn database_url() -> String {
    Lazy::force(&_DOTENV);

    env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("DATABASE_URL is not set in environment or .env file"))
}

pub fn jwt_secret() -> String {
    Lazy::force(&_DOTENV);

    env::var("JWT_SECRET")
        .unwrap_or_else(|_| panic!("JWT_SECRET is not set in environment or .env file"))
}
