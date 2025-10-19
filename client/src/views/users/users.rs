use dioxus::prelude::*;

use crate::{
    components::{CenteredInvisible, CenteredText},
    panels::LeftUsers,
    verify_user,
    views::View,
};

#[component]
pub fn ViewUsers() -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            left: rsx! { LeftUsers {} },
            right: rsx! { CenteredInvisible {
                CenteredText {
                    text: "Select a user to view their profile"
                }
            } }
        }
    }
}
