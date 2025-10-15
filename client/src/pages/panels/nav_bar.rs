use dioxus::prelude::*;

use crate::{
    components::IconButton,
    pages::{LeftPanel, PanelContext, RightPanel},
};

#[component]
pub fn NavBar() -> Element {
    let panel_context = use_context::<PanelContext>();

    let links = [
        ("chats", LeftPanel::Chats, asset!("assets/icons/chats.svg")),
        ("users", LeftPanel::Users, asset!("assets/icons/users.svg")),
        (
            "settings",
            LeftPanel::Settings,
            asset!("assets/icons/settings.svg"),
        ),
    ];

    rsx! {
        div {
            class: "mt-auto p-2 border-t border-gray-300 flex justify-around",

            { links.iter().map(|(id, panel, icon)| {
                let mut panel_context = panel_context.clone();
                let panel_clone = panel.clone();
                let id = id.to_string();
                let icon = icon.to_string();
                rsx! {
                    IconButton {
                        alt: "{id}",
                        icon: "{icon}",
                        onclick: move |_| {
                            panel_context.left.set(panel_clone.clone());
                            panel_context.right.set(RightPanel::Empty);
                        },
                    }
                }
            }) }
        }
    }
}
