use dioxus::prelude::*;

use crate::{
    pages::{LeftUsers, RightUser},
    views::View,
};

#[component]
pub fn ViewUser(username: Option<String>) -> Element {
    verify_user!();

    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftUsers {} },
            right: rsx! {
                { if let Some(username) = username.clone() {
                    RightUser { username }
                } else {
                    CenteredText {
                        text: "Select a user to view their profile"
                    }
                } }
            },
        }
    }
}
