use dioxus::prelude::*;

use crate::views::View;

#[component]
pub fn ViewSettingsAccount() -> Element {
    // verify_user!();

    rsx! {
        View {
            view_right: true,
            left: rsx! { p { "settings" } },
            right: rsx! { p { "account" } },
            // left: rsx! { LeftSettings {} },
            // right: rsx! { SettingsAccount {} },
        }
    }
}
