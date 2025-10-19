use dioxus::prelude::*;

use super::View;
use crate::verify_user;

#[component]
pub fn ViewUsers() -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            left: rsx! {
                div {
                    class: "p-4",
                    "Users"
                }
            },
            right: rsx! {}
        }
    }
}
