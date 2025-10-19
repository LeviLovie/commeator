mod chats;
mod groups;
mod home;
mod settings;
mod users;

pub use chats::*;
pub use groups::*;
pub use home::*;
pub use settings::*;
pub use users::*;

use dioxus::prelude::*;

use crate::{
    centrifugo::{CentrifugoClient, CentrifugoContext},
    components::NavBar,
    panels::{LayoutContext, PanelLayout},
};
use std::rc::Rc;

#[component]
pub fn View(view_right: bool, left: Element, right: Element) -> Element {
    let default_layout = use_signal(|| PanelLayout::Desktop);
    use_context_provider(|| LayoutContext {
        layout: default_layout,
    });

    let centrifugo = Rc::new(CentrifugoClient::new());
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

    use_effect({
        move || {
            let mut layout = use_context::<LayoutContext>().layout;
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
        }
    });

    let layout = use_context::<LayoutContext>().layout;

    match *layout.read() {
        PanelLayout::Desktop => rsx! {
            div {
                class: "flex h-[100vh] w-screen",

                div {
                    class: "flex flex-col border-r border-gray-300",

                    div {
                        class: "w-64 flex-1 overflow-y-auto",
                        {left}
                    }

                    NavBar {}
                }

                div {
                    class: "flex-1 overflow-y-auto",
                    {right}
                }
            }
        },
        PanelLayout::Mobile => rsx! {
            div {
                class: "flex flex-col h-[100dvh]",

                match view_right {
                    false => rsx! {
                        div {
                            class: "flex-1 overflow-y-auto",
                            {left}
                        }
                        div {
                            class: "sticky bottom-0 w-full bg-white border-t border-gray-300 z-50",
                            NavBar {}
                        }
                    },
                    true => rsx! {
                        div {
                            class: "flex-1 overflow-y-auto",
                            {right}
                        }
                        div {
                            class: "sticky bottom-0 w-full bg-white border-t border-gray-300 z-50",
                            NavBar {}
                        }
                    },
                }
            }
        },
    }
}
