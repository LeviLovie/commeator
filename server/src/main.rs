mod config;
mod db;
mod endpoints;
#[allow(unused_imports)]
mod entities;
mod error;
mod extractors;
mod jwt;

#[rocket::launch]
async fn rocket() -> _ {
    let db = db::init().await.expect("Failed to initialize database");

    let cors = rocket_cors::CorsOptions {
        allowed_origins: rocket_cors::AllOrSome::Some(rocket_cors::Origins {
            allow_null: false,
            exact: Some(std::collections::HashSet::from([config::CONFIG
                .url_web
                .clone()])),
            regex: None,
        }),
        allowed_methods: std::collections::HashSet::from([rocket::http::Method::Post.into()]),
        allowed_headers: rocket_cors::AllOrSome::Some(
            ["Content-Type", "Authorization"]
                .iter()
                .cloned()
                .map(|h| h.into())
                .collect(),
        ),
        send_wildcard: false,
        expose_headers: std::collections::HashSet::from(["Content-Type".into()]),
        max_age: Some(3600),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let config = rocket::Config {
        port: 4000,
        ..rocket::Config::default()
    };
    rocket::custom(config)
        .manage(db)
        .attach(cors)
        .mount("/g", endpoints::general::routes())
        .mount("/j", endpoints::jwt::routes())
        .mount("/u", endpoints::user::routes())
        .mount("/c", endpoints::chat::routes())
}
