use dioxus::prelude::*;

use crate::{components::IconButton, Route};

#[component]
pub fn Header(left: Element, center: Element, right: Element) -> Element {
    rsx! {
        div {
            class: "flex sticky top-0 bg-white z-10 items-center justify-between chat-header p-2 border-b border-gray-300",

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
pub fn HeaderButtonBack(route: Route) -> Element {
    let navigator = navigator();

    rsx! {
        IconButton {
            alt: "back",
            ty: "button",
            icon: asset!("assets/icons/back.svg"),
            onclick: move |_| {
                navigator.replace(route.clone());
            },
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
