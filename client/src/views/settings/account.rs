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
            view_right: true,
            left: rsx! { LeftSettings {} },
            right: rsx! { SettingsAccount {} },
        }
    }
}
