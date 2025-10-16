mod backend;
mod centrifugo;
mod components;
mod pages;
mod verify_user_jwt;

use dioxus::{logger::tracing::Level, prelude::*};

use pages::{AuthCallback, AuthError, AuthLogIn, AuthProfileSetup, Home};

#[derive(Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home,

    #[nest("/login")]
        #[route("/callback")] AuthCallback,

        #[route("/setup")]
        AuthProfileSetup,

        #[route("/login?:flow")]
        AuthLogIn { flow: String },

        #[route("/error?:id")]
        AuthError { id: String },
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to initialize logger");

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
