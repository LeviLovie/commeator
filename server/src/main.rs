mod chats;
mod db;
mod jwt;
mod messages;
mod users;
mod verify_kratos;
mod error;

#[allow(unused_imports)]
mod entities;

pub use db::db;
pub use jwt::verify_jwt;
pub use verify_kratos::verify_kratos_cookie;
pub use error::AppError;

use axum::{
    routing::{get, post}, Router
};
use tokio::net::TcpListener;

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
        .route(endpoints::jwt::G_GENERATE, get(jwt::endpoint_generate))
        .route(endpoints::jwt::G_VERIFY, get(jwt::endpoint_verify));

    app = app
        .route(endpoints::chats::G_LIST, get(chats::list_chats))
        .route(endpoints::chats::P_GET, post(chats::get_chat))
        .route(endpoints::chats::P_VERIFY_PRIVATE, post(chats::verify_private_chat));

    app = app
        .route(endpoints::messages::P_LIST, post(messages::list_messages))
        .route(endpoints::messages::P_SEND, post(messages::send_message));

    app = app
        .route(endpoints::users::G_ME, get(users::get_me))
        .route(endpoints::users::P_GET, post(users::get_user))
        .route(endpoints::users::P_SETUP, post(users::setup_user))
        .route(endpoints::users::P_LIST, post(users::list_users));

    #[cfg(debug_assertions)]
    {
        tracing::warn!("Debug mode is enabled. The /debug/user endpoint is available.");
        app = app.route("/debug/user", post(users::debug_user));
    }

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Starting server");
    axum::serve(listener, app).await.unwrap();
}
