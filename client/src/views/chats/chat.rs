use dioxus::prelude::*;

use crate::{
    panels::{LeftChats, RightChat},
    verify_user,
    views::View,
};

#[component]
pub fn ViewChat(uuid: Option<String>) -> Element {
    verify_user!();

    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftChats {} },
            right: rsx! {
                { if let Some(uuid) = uuid.clone() {
                    RightChat { uuid }
                } else {
                    CenteredText {
                        text: "Select a chat to start messeging"
                    }
                } }
            }
        }
    }
}
