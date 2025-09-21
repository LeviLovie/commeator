use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            h1 { "You are logged in!" }
            p { "Welcome to your messenger home." }
        }
    }
}
