use dioxus::prelude::*;

use crate::{
    components::{CenteredInvisible, CenteredText},
    pages::{LeftUsers, RightUser},
    views::View,
};

#[component]
pub fn ViewUser(username: String) -> Element {
    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftUsers {} },
            right: rsx! {
                { if username.is_empty() {
                    rsx! {
                        CenteredInvisible {
                            CenteredText {
                                text: "Select a user to view their profile"
                            }
                        }
                    }
                } else {
                    rsx! {
                        RightUser { username: username.clone() }
                    }
                } }
            },
        }
    }
}

#[component]
pub fn ViewUsers() -> Element {
    rsx! {
        ViewUser { username: "" }
    }
}
