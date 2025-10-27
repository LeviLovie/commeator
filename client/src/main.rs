#![allow(clippy::module_inception)]

mod auth;
mod backend;
mod centrifugo;
mod components;
mod panels;
mod views;

#[macro_use]
mod macros;

use dioxus::{logger::tracing::Level, prelude::*};

use auth::*;
use views::*;

use crate::{
    components::{CenteredForm, CenteredText},
};

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

            #[route("/redirect?:id")]
            AuthRedirect { id: String },

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

#[derive(Clone)]
#[cfg(not(target_arch = "wasm32"))]
pub struct JwtContext {
    pub jwt: Signal<Option<String>>,
}

#[component]
fn App() -> Element {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let jwt = use_signal(backend::local_storage::load_jwt);
        use_context_provider(|| JwtContext { jwt });
    }

    rsx! {
        document::Script {
            src: "https://cdn.tailwindcss.com",
        }

        Router::<Route> {}
    }
}

#[component]
#[cfg(not(target_arch = "wasm32"))]
fn RootLayout() -> Element {
    let jwt = use_context::<JwtContext>().jwt;

    use_effect(move || {
        let jwt_value = jwt();
        let mut jwt = jwt;
        spawn(async move {
            info!("Starting native authentication flow");
            if jwt_value.is_some() {
                info!("JWT already present in local storage, skipping native authentication");
                return;
            }

            let request_uuid = use_signal(uuid::Uuid::new_v4);

            spawn(async move {
                if let Err(e) = webbrowser::open(&utils::config::endpoints::auth::url_app_login(request_uuid().to_string()).await) {
                    error!("Failed to open web browser for login: {}", e);
                } else {
                    info!("Opened web browser for login");
                }
            });

            for i in 0..32 {
                match backend::natives_is_authenticated(request_uuid()).await {
                    Ok(Some(jwt_token)) => {
                        info!("Native authentication completed, received JWT");
                        backend::local_storage::save_jwt(&jwt_token);
                        info!("JWT saved to local storage");
                        navigator().replace(Route::ViewHome);
                        jwt.set(Some(jwt_token));
                        break;
                    }
                    Ok(None) => {
                        info!("Native authentication not yet completed, attempt {}/32", i + 1);
                    }
                    Err(e) => {
                        error!("Error checking native authentication: {}", e);
                    }
                }
                tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
            }
        });
    });

    match jwt() {
        Some(_) => rsx!{
            Outlet::<Route> {}
        },
        None => rsx! {
            CenteredForm {
                CenteredText {
                    text: "Please log in via the opened browser window."
                }
                // NotFullHeightSpinner {}
            }
        }
    }
}

#[component]
#[cfg(target_arch = "wasm32")]
fn RootLayout() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}
