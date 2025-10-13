use dioxus::prelude::*;

use crate::pages::{Item, PanelContext, RightPanel, SettingsPage};

#[component]
pub fn Settings() -> Element {
    let buttons = [("Account", SettingsPage::Account)];

    rsx! {
        div {
            { buttons.iter().map(|(name, page)| {
                let name = name.to_string();
                let page = *page;
                rsx! {
                    Item {
                        button {
                            class: "text-left p-2 w-full h-full hover:bg-gray-300 cursor-pointer",
                            onclick: move |_| {
                                let mut context = use_context::<PanelContext>();
                                context.settings_page.set(page);
                                context.right.set(RightPanel::Settings);
                            },

                            "{name}"
                        }
                    }
                }
            }) }
        }
    }
}
