use dioxus::prelude::*;

#[component]
pub fn Spinner() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen text-blue-500",

            img {
                class: "w-20 h-20",
                src: asset!("/assets/spinner.svg")
            }
        }
    }
}
