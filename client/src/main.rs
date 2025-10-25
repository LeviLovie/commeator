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
    #[layout(RootLayout)]
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

            #[route("/close")]
            AuthClose,

            #[route("/redirect")]
            AuthRedirect,

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

#[component]
fn RootLayout() -> Element {
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "desktop")]
    {
        use crate::backend::local_storage::save_jwt;
        use dioxus::desktop::{tao::event::Event, use_wry_event_handler};

        use_wry_event_handler(move |event, _| {
            if let Event::Opened { urls } = event {
                for url in urls {
                    if url.as_str().starts_with("commeator://callback") {
                        if let Some(query) = url.query() {
                            let args = query.split('&');
                            if let Some(jwt_pair) = args.clone().find(|arg| arg.starts_with("jwt="))
                            {
                                let jwt = jwt_pair.trim_start_matches("jwt=");
                                save_jwt(jwt);
                                navigator().replace(Route::ViewHome);
                            } else {
                                error!("[RootLayout] No JWT found in callback URL");
                            }
                        } else {
                            error!("No query found in URL");
                        }
                    }
                }
            }
        });
    }

    rsx! {
        Outlet::<Route> {}
    }
}
