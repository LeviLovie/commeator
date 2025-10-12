use dioxus::prelude::*;

#[component]
pub fn Empty() -> Element {
    rsx! {
        p { "Empty" }
    }
}
