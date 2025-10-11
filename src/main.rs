mod auth;
mod components;
mod pages;

use dioxus::{logger::tracing::Level, prelude::*};

use pages::{ErrorHandler, Home, LogIn};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home,
    #[route("/login?:flow")]
    LogIn { flow: String },
    #[route("/error?:id")]
    ErrorHandler { id: String },
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
