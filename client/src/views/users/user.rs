use dioxus::prelude::*;

use crate::{
    panels::{LeftUsers, RightUser},
    verify_user,
    views::View,
};

#[component]
pub fn ViewUser(username: String) -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            left: rsx! { LeftUsers {} },
            right: rsx! { RightUser { username } },
        }
    }
}
