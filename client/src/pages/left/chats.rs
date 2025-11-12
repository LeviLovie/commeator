use dioxus::prelude::*;

use crate::{
    Route,
    backend::{ApiData, list_chats, use_api_data},
    components::{Header, HeaderButton, HeaderText, Item, SmallIconButton, Spinner},
};
use utils::data::ChatInfo;

#[derive(Clone)]
pub struct ChatsContext {
    chats: Signal<ApiData<Vec<ChatInfo>>>,
}

#[component]
pub fn LeftChats() -> Element {
    {
        let chats = use_api_data(|| async { list_chats().await });
        let context = ChatsContext { chats };
        use_context_provider(|| context.clone());
    }

    let navigator = navigator();

    let context = use_context::<ChatsContext>();
    let chats = context.chats.read();
    if chats.is_loading() || chats.as_ref().is_none() {
        return rsx! { Spinner {} };
    }
    let chats = chats.as_ref().unwrap();

    rsx! {
        Header {
            left: rsx! {
                HeaderText { text: "Chats" }
            },
            center: rsx! {},
            right: rsx! {
                HeaderButton {
                    SmallIconButton {
                        alt: "New group".to_string(),
                        icon: asset!("/assets/icons/add.svg"),
                        ty: "button".to_string(),
                        onclick: move |_| {
                            navigator.replace(Route::ViewNewGroup);
                        },
                    }
                }
            },
        }

        { chats.iter().map(|chat| {
            let uuid = chat.uuid;
            rsx! {
                Item {
                    button {
                        class: "text-left p-2 w-full h-full hover:bg-gray-300 cursor-pointer",
                        onclick: move |_| {
                            navigator.replace(Route::ViewChat { uuid: uuid.to_string() });
                        },

                        "{chat.name}"
                    }
                }
            }
        }) }
    }
}
