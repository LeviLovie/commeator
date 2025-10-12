use dioxus::prelude::*;

use crate::{backend::chats::{list_chats, ChatInfo}, components::Spinner, pages::{state::jwt, PanelContext, PanelLayout, RightPanel}};

#[component]
pub fn Chats() -> Element {
    let mut context = use_context::<PanelContext>();

    use_effect(move || {
        if context.chats.read().is_none() {
            context.chats.set(Some(vec![]));
            spawn(async move {
                match list_chats(jwt().await).await {
                    Ok(c) => {
                        context.chats.set(Some(c));
                    }
                    Err(_) => {
                        error!("Failed to fetch chats");
                    },
                }
            });
        }
    });

    let chats = context.chats.read();

    match *chats {
        Some(ref chats) => rsx! {
            div {
                class: "chats-panel",

                { chats.iter().map(|chat| rsx! {
                    ChatItem { chat: chat.clone() }
                }) }
            }
        },
        None => rsx! {
            Spinner {}
        },
    }
}

#[component]
pub fn ChatItem(chat: ChatInfo) -> Element {
    let mut context = use_context::<PanelContext>();

    rsx! {
        button {
            class: "chat-item w-full mb-1 text-left p-2 bg-gray-200 hover:bg-gray-300 cursor-pointer",
            onclick: move |_| {
                context.right.set(RightPanel::Chat(chat.clone()));
            },

            "{chat.name}"
        }
    }
}
