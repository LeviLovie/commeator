mod auth;
mod backend;
mod components;
mod config;
mod pages;

use dioxus::{logger::tracing::Level, prelude::*};

use pages::{AuthCallback, AuthError, AuthLogIn, AuthProfileSetup, Home};

#[derive(Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home,

    #[nest("/auth")]
        #[route("/callback")]
        AuthCallback,

        #[route("/setup")]
        AuthProfileSetup,

        #[route("/login?:flow")]
        AuthLogIn { flow: String },

        #[route("/error?:id")]
        AuthError { id: String },
}

fn main() {
    let level = if cfg!(debug_assertions) {
        Level::INFO
    } else {
        Level::WARN
    };
    dioxus::logger::init(level).expect("failed to initialize logger");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Script {
            src: "https://cdn.tailwindcss.com",
        }

        Router::<Route> {}
    }
}
