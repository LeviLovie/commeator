use dioxus::prelude::*;

#[component]
pub fn Spinner() -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; justify-content: center; min-height: 100vh; color: #3b82f6;", // blue-500

            img {
                style: "width: 80px; height: 80px;",
                src: asset!("/assets/spinner.svg")
            }
        }
    }
}
