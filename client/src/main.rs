mod components;
mod pages;
mod services;
mod state;
mod views;

use dioxus::{logger::tracing::Level, prelude::*};

use pages::*;
use views::*;

#[derive(Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[end_nest]
    #[nest("/a")]
        #[route("/callback")]
        AuthCallback,

        #[route("/close")]
        AuthClose,

        #[route("/redirect?:id")]
        AuthRedirect { id: String },

        #[route("/setup")]
        AuthProfileSetup,

        #[route("/login?:flow")]
        AuthLogIn { flow: String },

        #[route("/error?:id")]
        #[end_nest]
        AuthError { id: String },

    #[layout(AppStateLayout)]
        #[route("/")]
        ViewHome,

        #[nest("/u")]
            #[route("/:username?")]
            ViewUser { username: Option<String> },

        #[end_nest]
        #[nest("/c")]
            #[route("/:uuid?")]
            ViewChat { uuid: Option<String> },

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
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("failed to initialize logger");

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
