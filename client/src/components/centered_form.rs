use dioxus::prelude::*;

#[component]
pub fn CenteredForm(children: Element) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen",

            div {
                class: "bg-white rounded-3xl shadow-xl p-10 w-full max-w-md",

                { &children }
            }
        }
    }
}
