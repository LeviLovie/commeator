use dioxus::prelude::*;

use crate::{
    pages::{LeftChats, RightNewGroup},
    views::View,
};

#[component]
pub fn ViewNewGroup() -> Element {
    rsx! {
        View {
            view_right: true,
            left: rsx! { LeftChats {} },
            right: rsx! { RightNewGroup { } },
        }
    }
}
