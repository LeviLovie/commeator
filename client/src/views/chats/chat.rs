use dioxus::prelude::*;

use crate::{
    panels::{LeftChats, RightChat},
    verify_user,
    views::View,
};

#[component]
pub fn ViewChat(uuid: String) -> Element {
    let _ = verify_user!();

    rsx! {
        View {
            left: rsx! { LeftChats {} },
            right: rsx! { RightChat { uuid } },
        }
    }
}
