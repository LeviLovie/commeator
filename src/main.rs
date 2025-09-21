mod components;
mod pages;

use dioxus::{logger::tracing::Level, prelude::*};

use pages::{ErrorHandler, Home, Landing, Registration};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Landing,
    #[route("/home")]
    Home,
    #[route("/registration?:flow")]
    Registration { flow: String },
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
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
        Router::<Route> {}
    }
}
