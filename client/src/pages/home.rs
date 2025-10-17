use dioxus::prelude::*;

use crate::{backend::get_kratos_user, components::Spinner, pages::Panels};

#[component]
pub fn Home() -> Element {
    let user = use_resource(|| async { get_kratos_user().await });
    if user().is_none() || user().as_ref().unwrap().is_none() {
        return rsx! { Spinner {} };
    }
    let user = user().as_ref().unwrap().as_ref().unwrap().clone();

    rsx! {
        div {
            class: "flex h-screen w-screen",

            Panels {}
        }
    }
}
