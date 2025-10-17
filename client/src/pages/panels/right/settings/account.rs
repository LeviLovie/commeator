use dioxus::prelude::*;

use crate::{
    components::{LogOut, Spinner},
    pages::PanelContext,
};

#[component]
pub fn SettingsAccount() -> Element {
    let user = use_context::<PanelContext>().user;

    if user().is_loading() {
        return rsx! {
            Spinner {}
        };
    }

    let user_guard = user.read();
    let user = user_guard.as_ref().unwrap().clone();

    rsx! {
        div {
            class: "p-4",

            p {
                class: "text-4xl font-bold mb-4",
                "Account"
            }

            div {
                p {
                    class: "text-s",
                    "{user.email}"
                }

                p {
                    class: "text-s",
                    "{user.nickname} ({user.nickname})"
                }

                LogOut {}
            }
        }
    }
}
