use dioxus::prelude::*;

use crate::{
    pages::{PanelContext, Panels},
    verify_user_jwt,
};

#[component]
pub fn Home() -> Element {
    let (_user, _jwt) = verify_user_jwt!();

    let panel_context = PanelContext::default();
    use_context_provider(|| panel_context.clone());

    rsx! {
        div {
            class: "flex h-screen w-screen",

            Panels {}
        }
    }
}
