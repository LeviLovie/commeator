use dioxus::prelude::*;

use crate::{pages::LeftUsers, views::View};

#[component]
pub fn ViewUser(username: String) -> Element {
    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftUsers {} },
            right: rsx! { p { "user: {username:?}" } },
            // right: rsx! {
            //     { if let Some(username) = username.clone() {
            //         RightUser { username }
            //     } else {
            //         CenteredText {
            //             text: "Select a user to view their profile"
            //         }
            //     } }
            // },
        }
    }
}

#[component]
pub fn ViewUsers() -> Element {
    rsx! {
        ViewUser { username: "" }
    }
}
