use dioxus::prelude::*;

use crate::{
    backend::{chats::ChatInfo, messages::{list_messages, Message}},
    components::{IconButton, Spinner},
    pages::{state::jwt, RightPanel},
};

#[component]
pub fn Chat() -> Element {
    let mut context = use_context::<crate::pages::PanelContext>();

    use_effect(move || {
        let messages = context.messages.read().clone();
        if !messages.0 && messages.1.is_none() {
            context.messages.set((true, None));

            let chat_info = context.chat.read().1.clone();
            let chat_id = if let Some(chat) = chat_info {
                chat.id
            } else {
                return;
            };

            spawn(async move {
                match list_messages(jwt().await, chat_id).await {
                    Ok(m) => {
                        context.messages.set((true, Some(m)));
                    }
                    Err(_) => {
                        error!("Failed to fetch chats");
                    },
                }
            });
        }
    });

    let chat_info = context.chat.read();
    let messages = context.messages.read();

    if !messages.0 || messages.1.is_none() || !chat_info.0 || chat_info.1.is_none() {
        return rsx! {
            Spinner {}
        };
    }

    let chat_info = chat_info.1.clone().unwrap();
    let chats = messages.1.clone().unwrap();

    rsx! {
        ChatHeader { chat_info: chat_info.clone() }

        { chats.iter().map(|msg| rsx! {
            div {
                class: "p-2 border-b border-gray-300",
                div {
                    class: "font-bold",
                    "User ID: {msg.sender_id}"
                }
                div {
                    "{msg.content}"
                }
                div {
                    class: "text-sm text-gray-500",
                    "Sent at: {msg.created_at}"
                }
            }
        }) }
    }
}

#[component]
pub fn ChatHeader(chat_info: ChatInfo) -> Element {
    let mut panel_context = use_context::<crate::pages::PanelContext>();

    rsx! {
        div {
            class: "flex chat-header p-2 border-b border-gray-300, text-center",

            IconButton {
                alt: "back",
                icon: asset!("assets/icons/back.svg"),
                onclick: move |_| {
                    panel_context.right.set(RightPanel::Empty);
                },
            }

            label {
                class: "w-full text-2xl font-bold",

                "{chat_info.name}"
            }
        }
    }
}
