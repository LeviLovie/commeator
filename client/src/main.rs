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
        #[route("/callback")] AuthCallback,

        #[route("/setup")]
        AuthProfileSetup,

        #[route("/login?:flow")]
        AuthLogIn { flow: String },

        #[route("/error?:id")]
        AuthError { id: String },
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::logger::initialize_default();
    dioxus_web::launch::launch(
        || dioxus_web::launch::launch(root, contexts, cfg),
        Vec::new(),
        Vec::new(),
    );
}

#[cfg(feature = "server")]
fn main() {
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
