use dioxus::prelude::*;

use super::View;
use crate::{components::Spinner, verify_user, Route};

#[component]
pub fn ViewHome() -> Element {
    let _ = verify_user!();
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
