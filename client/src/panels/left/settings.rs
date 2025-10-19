use dioxus::prelude::*;

use crate::{
    Route,
    components::{Header, HeaderText, Item},
};

#[component]
pub fn LeftSettings() -> Element {
    let navigator = navigator();

    let buttons = [("Account", Route::ViewSettingsAccount)];

    rsx! {
        div {
            Header {
                left: rsx! {
                    HeaderText { text: "Settings" }
                },
                center: rsx! {},
                right: rsx! {},
            }

            { buttons.iter().map(|(name, route)| {
                let name = name.to_string();
                let route = route.clone();
                rsx! {
                    Item {
                        button {
                            class: "text-left p-2 w-full h-full hover:bg-gray-300 cursor-pointer",
                            onclick: move |_| {
                                navigator.push(route.clone());
                            },

                            "{name}"
                        }
                    }
                }
            }) }
        }
    }
}
