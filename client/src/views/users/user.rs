use dioxus::prelude::*;

use crate::views::View;

#[component]
pub fn ViewUser(username: String) -> Element {
    // verify_user!();

    rsx! {
        View {
            view_right: true,
            left: rsx! { p { "users" } },
            right: rsx! { p { "user: {username}" } },
            // left: rsx! { LeftUsers {} },
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
