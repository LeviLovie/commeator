use dioxus::prelude::*;

use super::View;
use crate::verify_user;

#[component]
pub fn ViewHome() -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            left: rsx! {},
            right: rsx! {
                div {
                    class: "p-4",
                    "Welcome to the Home View!"
                }
            }
        }
    }
}
