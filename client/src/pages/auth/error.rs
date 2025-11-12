use dioxus::prelude::*;

#[component]
pub fn AuthError(id: String) -> Element {
    rsx! {
        div {
            h1 { "Oops! Something went wrong." }
            p { "ErrorHandler ID: {id}" }
            p { "Please try again or contact support." }
        }
    }
}
