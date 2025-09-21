use dioxus::prelude::*;

#[component]
pub fn Landing() -> Element {
    rsx! {
        div { id: "landing",
            h1 { "Welcome to Messenger!" }
            div { id: "buttons",
                a {
                    href: "http://localhost:4433/self-service/registration/browser",
                    "Sign Up"
                }
                a {
                    href: "http://localhost:4433/self-service/login/browser",
                    "Log In"
                }
            }
        }
    }
}
