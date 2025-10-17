mod api_data;
mod left;
mod nav_bar;
mod right;

pub use api_data::ApiData;
pub use left::*;
pub use nav_bar::NavBar;
pub use right::*;

use dioxus::prelude::*;

use crate::{backend::my_user, pages::panels::api_data::use_api_data};
use utils::requests::UserInfo;

#[derive(Clone)]
#[allow(dead_code)]
pub enum PanelLayout {
    Desktop,
    Mobile,
}

#[derive(Clone)]
pub struct PanelContext {
    pub left: Signal<LeftPanel>,
    pub right: Signal<RightPanel>,
    pub user: Signal<ApiData<UserInfo>>,
    pub settings_page: Signal<SettingsPage>,
}

#[component]
pub fn Panels() -> Element {
    let mut layout = use_signal(|| PanelLayout::Desktop);

    let left = use_signal(|| LeftPanel::Chats);
    let right = use_signal(|| RightPanel::Empty);
    let user = use_api_data(|| async { my_user().await });
    let settings_page = use_signal(|| SettingsPage::Empty);

    let context = PanelContext {
        left,
        right,
        user,
        settings_page,
    };

    use_context_provider(|| context.clone());

    use_effect(move || {
        let width = web_sys::window()
            .unwrap()
            .inner_width()
            .unwrap()
            .as_f64()
            .unwrap();
        layout.set(if width >= 768.0 {
            PanelLayout::Desktop
        } else {
            PanelLayout::Mobile
        });
    });

    match layout().clone() {
        PanelLayout::Desktop => rsx! {
            div {
                class: "bg-gray-100 border-r border-gray-300 flex flex-col w-64 md:w-64 shrink-0 md:flex hidden md:flex",
                LeftPanelWrapper {}
                NavBar {}
            }

            div {
                class: "flex-1 bg-white overflow-auto",
                RightPanelWrapper {}
            }
        },
        PanelLayout::Mobile => rsx! {
            div {
                class: "flex-1 flex-col h-screen md:hidden",

                { if !matches!(*context.right.read(), RightPanel::Empty) {
                    rsx! {
                        div {
                            class: "flex-1 overflow-auto",
                            RightPanelWrapper {}
                        }
                    }
                } else {
                    rsx! {
                        div {
                            class: "flex-1 relative h-full",

                            div {
                                class: "bg-gray-100 border-gray-300 flex flex-col md:flex",
                                LeftPanelWrapper {}
                            }

                            div {
                                class: "absolute bottom-0 left-0 w-full h-16 bg-white border-t border-gray-300 pb-[env(safe-area-inset-bottom)]",
                                NavBar {}
                            }
                        }
                    }
                } }
            }
        },
    }
}
