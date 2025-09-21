use dioxus::prelude::*;

#[component]
pub fn CenteredForm(children: Element) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen bg-gray-100",
            div {
                class: "bg-white rounded-2xl shadow-xl p-8 w-full max-w-md",
                { &children }
            }
        }
    }
}
