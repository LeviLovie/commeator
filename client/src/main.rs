#![allow(clippy::module_inception)]

mod auth;
mod backend;
mod centrifugo;
mod components;
mod macros;
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

    #[nest("/u")]
        #[route("/")]
        ViewUsers,

        #[route("/:username")]
        ViewUser { username: String },

    #[end_nest]
    #[nest("/c")]
        #[route("/")]
        ViewChats,

        #[route("/:uuid")]
        ViewChat { uuid: String },

    #[end_nest]
    #[nest("/g")]
        #[route("/new")]
        ViewNewGroup,

    #[end_nest]
    #[nest("/s")]
        #[route("/")]
        ViewSettings,

        #[route("/account")]
        ViewSettingsAccount,

    #[end_nest]
    #[nest("/a")]
        #[route("/callback")]
        AuthCallback,

        #[route("/setup")]
        AuthProfileSetup,

        #[route("/login?:flow")]
        AuthLogIn { flow: String },

        #[route("/error?:id")]
        #[end_nest]
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
