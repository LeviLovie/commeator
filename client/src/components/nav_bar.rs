use dioxus::prelude::*;

use crate::{Route, components::IconButton};

#[component]
pub fn NavBar() -> Element {
    let navigator = navigator();

    let links = [
        ("chats", Route::ViewChats, asset!("assets/icons/chats.svg")),
        ("users", Route::ViewUsers, asset!("assets/icons/users.svg")),
        (
            "settings",
            Route::ViewSettings,
            asset!("assets/icons/settings.svg"),
        ),
    ];

    rsx! {
        div {
            class: "mt-auto p-2 border-t border-gray-300 flex justify-around",

            { links.iter().map(|(id, route, icon)| {
                let id = id.to_string();
                let route = route.clone();
                let icon = icon.to_string();
                rsx! {
                    IconButton {
                        alt: "{id}",
                        icon: "{icon}",
                        ty: "button".to_string(),
                        onclick: move |_| {
                            navigator.replace(route.clone());
                        },
                    }
                }
            }) }
        }
    }
}
