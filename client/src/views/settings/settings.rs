use dioxus::prelude::*;

use crate::views::View;

#[component]
pub fn ViewSettings() -> Element {
    // verify_user!();

    rsx! {
        View {
            view_right: false,
            left: rsx! { p { "settings" } },
            right: rsx! { p { "empty" } },
            // left: rsx! { LeftSettings {} },
            // right: rsx! { CenteredInvisible {
            //     CenteredText {
            //         text: "Select a settings category to edit"
            //     }
            // } },
        }
    }
}
