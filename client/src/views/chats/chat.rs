use dioxus::prelude::*;

use crate::{
    panels::{LeftChats, RightChat},
    verify_user,
    views::View,
};

#[component]
pub fn ViewChat(uuid: String) -> Element {
    verify_user!();

    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftChats {} },
            right: rsx! { RightChat { uuid } },
        }
    }
}
