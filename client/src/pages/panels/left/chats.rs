use dioxus::prelude::*;

use crate::{
    backend::list_chats,
    components::{SmallIconButton, Spinner},
    pages::{panels::api_data::use_api_data, ApiData, Item, PanelContext, RightPanel},
};
use utils::requests::ChatInfo;

#[derive(Clone)]
pub struct ChatsContext {
    chats: Signal<ApiData<Vec<ChatInfo>>>,
}

#[component]
pub fn Chats() -> Element {
    {
        let chats = use_api_data(|| async { list_chats().await });
        let context = ChatsContext { chats };
        use_context_provider(|| context.clone());
    }

    let context = use_context::<ChatsContext>();
    let chats = context.chats.read();
    if chats.is_loading() || chats.as_ref().is_none() {
        return rsx! { Spinner {} };
    }
    let chats = chats.as_ref().unwrap();

    rsx! {
        div {
            div {
                class: "flex justify-between p-2",

                p {
                    class: "text-m p-0",
                    "Chats"
                }

                SmallIconButton {
                    alt: "New group".to_string(),
                    icon: asset!("/assets/icons/add.svg"),
                    ty: "button".to_string(),
                    onclick: move |_| {
                        use_context::<PanelContext>().right.set(RightPanel::NewGroup);
                    },
                }
            }

            { chats.iter().map(|chat| {
                let uuid = chat.uuid;
                rsx! {
                    Item {
                        button {
                            class: "text-left p-2 w-full h-full hover:bg-gray-300 cursor-pointer",
                            onclick: move |_| {
                                use_context::<PanelContext>().right.set(RightPanel::Chat(uuid));
                            },

                            "{chat.name}"
                        }
                    }
                }
            }) }
        }
    }
}
