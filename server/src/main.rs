mod chats;
mod conn;
mod error;
mod jwt;
mod messages;
mod users;
mod verify_kratos;

#[allow(unused_imports)]
mod entities;

pub use conn::db;
pub use error::AppError;
pub use jwt::verify_jwt;
pub use verify_kratos::verify_kratos_cookie;

use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE, COOKIE},
        Method,
    },
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use utils::config::endpoints;

#[allow(unused_imports)]
mod schema {
    pub use super::entities::prelude::*;
    pub use super::entities::*;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut app = Router::new();

    app = app
        .route(endpoints::jwt::IG_GENERATE, get(jwt::endpoint_generate))
        .route(endpoints::jwt::IG_VERIFY, get(jwt::endpoint_verify))
        .route(
            endpoints::jwt::IG_GENERATE_CENTRIFUGO,
            get(jwt::endpoint_generate_centrifugo),
        );

    app = app
        .route(endpoints::chats::IG_LIST, get(chats::list_chats))
        .route(endpoints::chats::IP_GET, post(chats::get_chat))
        .route(
            endpoints::chats::IP_VERIFY_PRIVATE,
            post(chats::verify_private_chat),
        )
        .route(endpoints::groups::IP_NEW, post(chats::new_group));

    app = app
        .route(endpoints::messages::IP_LIST, post(messages::list_messages))
        .route(endpoints::messages::IP_SEND, post(messages::send_message))
        .route(
            endpoints::messages::IP_DELETE,
            post(messages::delete_message),
        )
        .route(endpoints::messages::IP_EDIT, post(messages::edit_message));

    app = app
        .route(endpoints::users::IG_CHECK, get(users::check_user))
        .route(endpoints::users::IG_ME, get(users::get_me))
        .route(endpoints::users::IP_GET, post(users::get_user))
        .route(endpoints::users::IP_SETUP, post(users::setup_user))
        .route(endpoints::users::IP_LIST, post(users::list_users))
        .route(endpoints::users::IP_CHAT, post(users::chat_users));

    #[cfg(debug_assertions)]
    {
        tracing::warn!("Debug mode is enabled. The /debug/user endpoint is available.");
        app = app.route("/debug/user", post(users::debug_user));
    }

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:8000"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, COOKIE])
        .allow_credentials(true);

    app = app.layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Starting server");
    axum::serve(listener, app).await.unwrap();
}
