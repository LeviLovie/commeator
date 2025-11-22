mod components;
mod config;
mod fetch;
mod pages;
mod request;
#[macro_use]
mod macros;
mod services;
mod state;
mod views;

use dioxus::{logger::tracing::Level, prelude::*};

use pages::*;
use state::AppStateLayout;
use views::*;

#[derive(Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[nest("/a")]
        #[route("/close")]
        AuthClose,

        #[route("/redirect?:id")]
        AuthRedirect { id: String },

        #[route("/login?:flow")]
        AuthLogIn { flow: String },

        #[route("/error?:id")]
        AuthError { id: String },

        #[route("/callback")]
        AuthCallback,
    #[end_nest]

    #[layout(AppStateLayout)]
        #[route("/")]
        ViewHome,

        #[route("/a/setup")]
        AuthProfileSetup,

        #[nest("/u")]
            #[route("/")]
            ViewUsers {},

            #[route("/:username")]
            ViewUser { username: String },
        #[end_nest]

        #[nest("/c")]
            #[route("/")]
            ViewChats {},

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
        // #[end_nest]
}

fn main() {
    dioxus::logger::init(Level::DEBUG).expect("failed to initialize logger");

    #[cfg(feature = "desktop")]
    {
        use dioxus_desktop::{Config, WindowBuilder};

        LaunchBuilder::new()
            .with_cfg(desktop! {
               Config::new().with_window(
                   WindowBuilder::new()
                       .with_title("Commeator")
               )
            })
            .launch(App);
    }

    #[cfg(not(feature = "desktop"))]
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
