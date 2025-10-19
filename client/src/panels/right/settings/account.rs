use dioxus::prelude::*;

use crate::{
    backend::{my_user, use_api_data},
    components::{Header, HeaderButtonBack, HeaderText, LogOut, Spinner},
    verify_user, Route,
};

#[component]
pub fn SettingsAccount() -> Element {
    let _ = verify_user!();

    let user = use_api_data(|| async { my_user().await });
    if user.read().is_loading() || user.read().as_ref().is_none() {
        return rsx! { Spinner {} };
    }

    let user_guard = user.read();
    let user = user_guard.as_ref().unwrap().clone();

    rsx! {
        Header {
            left: rsx! { HeaderButtonBack {
                route: Route::ViewSettings,
            } },
            center: rsx! { HeaderText {
                text: "Account"
            } },
            right: rsx! {}
        }

        div {
            class: "flex flex-col p-4 space-y-4",

            p {
                class: "text-s",
                "{user.nickname} @{user.username}"
            }

            LogOut {}
        }
    }
}
