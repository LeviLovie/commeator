use dioxus::prelude::*;

use crate::{
    panels::{LeftChats, RightNewGroup},
    verify_user,
    views::View,
};

#[component]
pub fn ViewNewGroup() -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftChats {} },
            right: rsx! { RightNewGroup { } },
        }
    }
}
