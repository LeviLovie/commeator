use dioxus::prelude::*;

use crate::{
    panels::{LeftSettings, SettingsAccount},
    verify_user,
    views::View,
};

#[component]
pub fn ViewSettingsAccount() -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            left: rsx! { LeftSettings {} },
            right: rsx! { SettingsAccount {} },
        }
    }
}
