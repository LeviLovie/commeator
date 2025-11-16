use dioxus::prelude::*;

use crate::{
    pages::{LeftSettings, SettingsAccount},
    views::View,
};

#[component]
pub fn ViewSettingsAccount() -> Element {
    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftSettings {} },
            right: rsx! { SettingsAccount {} },
        }
    }
}
