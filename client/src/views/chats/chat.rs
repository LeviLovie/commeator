use dioxus::prelude::*;

use crate::{
    components::{CenteredInvisible, CenteredText},
    pages::{LeftChats, RightChat},
    views::View,
};

#[component]
pub fn ViewChat(uuid: String) -> Element {
    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftChats {} },
            right: rsx! {
                RightChat { uuid }
            }
        }
    }
}

#[component]
pub fn ViewChats() -> Element {
    rsx! {
        View {
            view_right: false,
            left: rsx! { LeftChats {} },
            right: rsx! {
                CenteredInvisible {
                    CenteredText {
                        text: "Select a chat to start messeging"
                    }
                }
            }
        }
    }
}
