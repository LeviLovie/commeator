use dioxus::prelude::*;

use crate::{
    components::{CenteredInvisible, CenteredText},
    pages::LeftSettings,
    views::View,
};

#[component]
pub fn ViewSettings() -> Element {
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
