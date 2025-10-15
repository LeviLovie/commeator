use dioxus::prelude::*;

use crate::{pages::Panels, verify_user_jwt};

#[component]
pub fn Home() -> Element {
    let (_user, _jwt) = verify_user_jwt!();

    rsx! {
        div {
            class: "flex h-screen w-screen",

            Panels {}
        }
    }
}
