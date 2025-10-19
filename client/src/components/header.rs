use dioxus::prelude::*;

#[component]
pub fn Header(left: Element, center: Element, right: Element) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between chat-header p-2 border-b border-gray-300",

            div {
                class: "flex w-8",
                {left}
            }

            div {
                {center}
            }

            div {
                class: "flex w-8",
                {right}
            }
        }
    }
}

#[component]
pub fn HeaderButton(children: Element) -> Element {
    rsx! {
        div {
            class: "w-8 h-8 flex items-center justify-center",
            {children}
        }
    }
}

#[component]
pub fn HeaderText(text: String) -> Element {
    rsx! {
        label {
            class: "text-2xl font-bold",
            "{text}"
        }
    }
}
