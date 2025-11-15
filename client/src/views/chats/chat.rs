use dioxus::prelude::*;

use crate::views::View;

#[component]
pub fn ViewChat(uuid: String) -> Element {
    // verify_user!();

    rsx! {
        View {
            view_right: true,
            left: rsx! { p { "chats" } },
            right: rsx! { p { "chat: {uuid}" } },
            // left: rsx! { LeftChats {} },
            // right: rsx! {
            //     { if let Some(uuid) = uuid.clone() {
            //         RightChat { uuid }
            //     } else {
            //         CenteredText {
            //             text: "Select a chat to start messeging"
            //         }
            //     } }
            // }
        }
    }
}
