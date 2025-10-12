use dioxus::prelude::*;

use crate::{backend::chats::{list_chats, ChatInfo}, components::Spinner, pages::{state::jwt, PanelContext, RightPanel}};

#[component]
pub fn Chats() -> Element {
    let mut context = use_context::<PanelContext>();

    use_effect(move || {
        let chats = context.chats.read().clone();
        if !chats.0 && chats.1.is_none() {
            context.chats.set((true, None));
            spawn(async move {
                match list_chats(jwt().await).await {
                    Ok(c) => {
                        context.chats.set((true, Some(c)));
                    }
                    Err(_) => {
                        error!("Failed to fetch chats");
                    },
                }
            });
        }
    });

    let chats = context.chats.read();

    if !chats.0 || chats.1.is_none() {
        return rsx! {
            Spinner {}
        };
    }

    let chats = chats.1.clone().unwrap();

    rsx! {
        div {
            class: "chats-panel",

            { chats.iter().map(|chat| rsx! {
                ChatItem { chat: chat.clone() }
            }) }
        }
    }
}

#[component]
pub fn ChatItem(chat: ChatInfo) -> Element {
    let mut context = use_context::<PanelContext>();

    rsx! {
        button {
            class: "chat-item w-full mb-1 text-left p-2 bg-gray-200 hover:bg-gray-300 cursor-pointer",
            onclick: move |_| {
                context.messages.set((false, None));
                context.chat.set((true, Some(chat.clone())));
                context.right.set(RightPanel::Chat);
            },

            "{chat.name}"
        }
    }
}
