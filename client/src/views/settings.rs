use dioxus::prelude::*;

use super::View;
use crate::verify_user;

#[component]
pub fn ViewSettings() -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            left: rsx! {
                div {
                    class: "p-4",
                    "Settings"
                }
            },
            right: rsx! {}
        }
    }
}
