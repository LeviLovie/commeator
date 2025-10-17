use dioxus::prelude::*;

#[component]
pub fn CenteredForm(children: Element) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen",

            div {
                class: "flex justify-center items-start min-h-screen pt-[15vh] w-full",

                div {
                    class: "bg-white rounded-3xl shadow-xl p-6 w-full max-w-md",

                    {children}
                }
            }
        }
    }
}
