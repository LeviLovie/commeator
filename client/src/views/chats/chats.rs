use dioxus::prelude::*;

use crate::{
    components::{CenteredInvisible, CenteredText},
    panels::LeftChats,
    verify_user,
    views::View,
};

#[component]
pub fn ViewChats() -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            left: rsx! { LeftChats {} },
            right: rsx! { CenteredInvisible {
                CenteredText {
                    text: "Select a chat to start messaging"
                }
            } }
        }
    }
}
