mod auth;
mod backend;
mod centrifugo;
mod components;
mod panels;
mod views;

use dioxus::{logger::tracing::Level, prelude::*};

use auth::*;
use views::*;

#[derive(Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    ViewHome,

    #[route("/u")]
    ViewUsers,

    #[route("/c")]
    ViewChats,

    #[route("/s")]
    ViewSettings,

    #[nest("/a")]
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
