mod api_data;
mod left;
mod nav_bar;
mod right;

pub use api_data::ApiData;
pub use left::*;
pub use nav_bar::NavBar;
pub use right::*;

use dioxus::prelude::*;
use std::sync::Arc;

use crate::{
    backend::my_user, centrifugo::CentrifugoClient, pages::panels::api_data::use_api_data,
};
use utils::{data::UserInfo, updates::Update};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Clone)]
pub struct LayoutContext {
    pub layout: Signal<PanelLayout>,
}

#[derive(Clone)]
pub struct CentrifugoContext {
    pub client: Arc<CentrifugoClient>,
}

#[component]
pub fn Panels() -> Element {
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

    let default_layout = use_signal(|| PanelLayout::Desktop);
    use_context_provider(|| LayoutContext {
        layout: default_layout,
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let centrifugo = Arc::new(CentrifugoClient::new());
    use_future({
        let centrifugo = centrifugo.clone();
        move || {
            let centrifugo = centrifugo.clone();
            async move {
                if let Err(e) = centrifugo.connect().await {
                    error!("Failed to connect to Centrifugo: {e}");
                }
            }
        }
    });
    use_context_provider(|| CentrifugoContext {
        client: centrifugo.clone(),
    });

    let default_updates: ChatUpdatesSignal = use_signal(Vec::new);
    use_context_provider(|| ChatUpdatesContext(default_updates));

    use_effect(move || {
        // let mut layout = use_context::<LayoutContext>().layout;
        // let width = web_sys::window()
        //     .unwrap()
        //     .inner_width()
        //     .unwrap()
        //     .as_f64()
        //     .unwrap();
        // layout.set(if width >= 768.0 {
        //     PanelLayout::Desktop
        // } else {
        //     PanelLayout::Mobile
        // });
    });

    let layout = use_context::<LayoutContext>().layout;

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
