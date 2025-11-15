use dioxus::prelude::*;

use crate::views::View;

#[component]
pub fn ViewNewGroup() -> Element {
    // verify_user!();

    rsx! {
        View {
            view_right: true,
            left: rsx! { p { "chats" } },
            right: rsx! { p { "new group" } },
            // left: rsx! { LeftChats {} },
            // right: rsx! { RightNewGroup { } },
        }
    }
}
