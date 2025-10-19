use dioxus::prelude::*;

use super::View;
use crate::{Route, components::Spinner, verify_user};

#[component]
pub fn ViewHome() -> Element {
    let _ = verify_user!();
    let navigator = navigator();

    use_effect(move || {
        navigator.replace(Route::ViewChats);
    });

    rsx! {
        View {
            left: rsx! { Spinner {} },
            right: rsx! { Spinner {} }
        }
    }
}
