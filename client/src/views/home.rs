use dioxus::prelude::*;

use super::View;
use crate::{Route, components::Spinner, verify_user};

#[component]
pub fn ViewHome() -> Element {
    verify_user!();
    let navigator = navigator();

    use_effect(move || {
        navigator.replace(Route::ViewChats);
    });

    rsx! {
        View {
            view_right: false,
            left: rsx! { Spinner {} },
            right: rsx! { Spinner {} }
        }
    }
}
