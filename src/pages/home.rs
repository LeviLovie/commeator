use dioxus::prelude::*;

use crate::{
    components::{CenteredForm, LogOut},
    verify_user_jwt,
};

#[component]
pub fn Home() -> Element {
    let (user, _jwt) = verify_user_jwt!();

    rsx! {
        CenteredForm {
            p {
                class: "text-4xl font-bold text-gray-800 mb-4",
                "Welcome, {user.identity.traits.email}!"
            }

            LogOut {}
        }
    }
}
