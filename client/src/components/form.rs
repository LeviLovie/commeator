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

#[component]
pub fn CenteredInvisible(children: Element) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center min-h-screen",

            div {
                class: "flex justify-center items-start min-h-screen pt-[35vh] w-full",

                div {
                    class: "p-6 w-full max-w-md",

                    {children}
                }
            }
        }
    }
}

#[component]
pub fn CenteredText(text: String) -> Element {
    rsx! {
        p {
            class: "text-center text-gray-500 text-sm",
            "{text}"
        }
    }
}

#[component]
pub fn Error(text: String) -> Element {
    rsx! {
        CenteredForm {
            p {
                class: "text-center font-bold text-red-500 text-sm",
                "{text}"
            }
        }
    }
}
