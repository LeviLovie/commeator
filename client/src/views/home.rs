use dioxus::prelude::*;

use super::View;
use crate::{components::Spinner, Route};

#[component]
pub fn ViewHome() -> Element {
    let navigator = navigator();

    use_effect(move || {
        navigator.replace(Route::ViewChats {});
    });

    rsx! {
        View {
            view_right: false,
            left: rsx! { Spinner {} },
            right: rsx! { Spinner {} }
        }
    }
}
