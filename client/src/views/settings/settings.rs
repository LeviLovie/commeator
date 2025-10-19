use dioxus::prelude::*;

use crate::{
    components::{CenteredInvisible, CenteredText},
    panels::LeftSettings,
    verify_user,
    views::View,
};

#[component]
pub fn ViewSettings() -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            view_right: false,
            left: rsx! { LeftSettings {} },
            right: rsx! { CenteredInvisible {
                CenteredText {
                    text: "Select a settings category to edit"
                }
            } },
        }
    }
}
