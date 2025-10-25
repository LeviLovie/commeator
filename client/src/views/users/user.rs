use dioxus::prelude::*;

use crate::{
    panels::{LeftUsers, RightUser},
    verify_user,
    views::View,
};

#[component]
pub fn ViewUser(username: String) -> Element {
    verify_user!();

    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftUsers {} },
            right: rsx! { RightUser { username } },
        }
    }
}
