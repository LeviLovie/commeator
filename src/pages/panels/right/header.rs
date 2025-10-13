use dioxus::prelude::*;

use crate::{
    components::IconButton,
    pages::{PanelContext, RightPanel},
};

#[component]
pub fn Header(title: String) -> Element {
    let mut panel_state = use_context::<PanelContext>();

    rsx! {
        div {
            class: "flex items-center justify-between chat-header p-2 border-b border-gray-300",

            div {
                class: "flex w-8",
                IconButton {
                    alt: "back",
                    icon: asset!("assets/icons/back.svg"),
                    onclick: move |_| {
                        panel_state.right.set(RightPanel::Empty);
                    },
                }
            }

            label {
                class: "text-2xl font-bold",
                "{title}"
            }

            div {
                class: "flex w-8",
            }
        }
    }
}
