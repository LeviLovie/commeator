use dioxus::prelude::*;

use crate::{
    backend::chats::{list_chats, ChatInfo},
    components::Spinner,
    pages::{panels::api_data::use_api_data, state::jwt, ApiData, Item, PanelContext, RightPanel},
};

#[derive(Clone)]
pub struct ChatsContext {
    chats: Signal<ApiData<Vec<ChatInfo>>>,
}

#[component]
pub fn Chats() -> Element {
    {
        let chats = use_api_data(|| async { list_chats(jwt().await).await });
        let context = ChatsContext { chats };
        use_context_provider(|| context.clone());
    }

    let context = use_context::<ChatsContext>();
    let chats = context.chats.read();
    if chats.is_loading() {
        return rsx! { Spinner {} };
    }
    let chats = chats.as_ref().unwrap();

    rsx! {
        div {
            { chats.iter().map(|chat| {
                let id = chat.id;
                rsx! {
                    Item {
                        button {
                            class: "text-left p-2 w-full h-full hover:bg-gray-300 cursor-pointer",
                            onclick: move |_| {
                                use_context::<PanelContext>().right.set(RightPanel::Chat(id));
                            },

                            "{chat.name}"
                        }
                    }
                }
            }) }
        }
    }
}
