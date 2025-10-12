mod chat;
mod chats;
mod empty;
mod nav_bar;
mod profile;
mod settings;
mod settings_tab;
mod users;

pub use chat::Chat;
pub use chats::Chats;
pub use empty::Empty;
pub use nav_bar::NavBar;
pub use profile::Profile;
pub use settings::Settings;
pub use settings_tab::SettingsTab;
pub use users::Users;

use dioxus::prelude::*;

use crate::backend::{chats::ChatInfo, messages::Message};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LeftPanel {
    Chats,
    Users,
    Settings,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RightPanel {
    Empty,
    Chat,
    SettingsTab,
    UserProfile(i32),
}

#[derive(Clone)]
pub enum PanelLayout {
    Desktop,
    Mobile,
}

impl LeftPanel {
    pub fn component(&self) -> Element {
        rsx! {
            div {
                class: "p-1",

                match self {
                    LeftPanel::Chats => rsx! { Chats {} },
                    LeftPanel::Users => rsx! { Users {} },
                    LeftPanel::Settings => rsx! { Settings {} },
                }
            }
        }
    }
}

impl RightPanel {
    pub fn component(&self) -> Element {
        rsx! {
            div {
                class: "p-1",

                match self {
                    RightPanel::Empty => rsx! { Empty {} },
                    RightPanel::Chat => rsx! { Chat { } },
                    RightPanel::SettingsTab => rsx! { SettingsTab {} },
                    RightPanel::UserProfile(_user_id) => rsx! { Profile {} },
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PanelContext {
    pub left: Signal<LeftPanel>,
    pub right: Signal<RightPanel>,
    pub layout: Signal<PanelLayout>,
    pub chats: Signal<(bool, Option<Vec<ChatInfo>>)>,
    pub chat: Signal<(bool, Option<ChatInfo>)>,
    pub messages: Signal<(bool, Option<Vec<Message>>)>,
}

impl Default for PanelContext {
    fn default() -> Self {
        Self {
            left: use_signal(|| LeftPanel::Chats),
            right: use_signal(|| RightPanel::Empty),
            layout: use_signal(|| PanelLayout::Desktop),
            chats: use_signal(|| (false, None)),
            chat: use_signal(|| (false, None)),
            messages: use_signal(|| (false, None)),
        }
    }
}

#[component]
pub fn Panels() -> Element {
    let panel_context = use_context::<PanelContext>();

    #[cfg(target_arch = "wasm32")]
    {
        let mut context = use_context::<PanelContext>();
        use_effect(move || {
            let width = web_sys::window()
                .unwrap()
                .inner_width()
                .unwrap()
                .as_f64()
                .unwrap();
            context.layout.set(if width >= 768.0 {
                PanelLayout::Desktop
            } else {
                PanelLayout::Mobile
            });
        });
    }

    let layout = panel_context.layout.read().clone();
    match layout {
        PanelLayout::Desktop => rsx! {
            div {
                class: "bg-gray-100 border-r border-gray-300 flex flex-col
                        w-64 md:w-64 shrink-0
                        md:flex
                        hidden md:flex",

                { panel_context.left.read().component() }

                NavBar {}
            }

            div {
                class: "flex-1 bg-white overflow-auto",

                { panel_context.right.read().component() }
            }
        },
        PanelLayout::Mobile => rsx! {
            div {
                class: "flex-1 bg-white overflow-auto md:hidden flex",

                { if *panel_context.right.read() != RightPanel::Empty {
                    rsx! {
                        { panel_context.right.read().component() }
                    }
                } else {
                    rsx! {
                        div {
                            class: "flex-1 bg-white overflow-auto",

                            { panel_context.left.read().component() }

                            NavBar {}
                        }
                    }
                }}
            }
        },
    }
}
