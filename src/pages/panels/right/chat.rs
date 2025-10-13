use dioxus::prelude::*;

use crate::{
    backend::{
        chats::{get_chat, ChatInfo},
        messages::{list_messages, MessageInfo},
    },
    components::{IconButton, Spinner},
    pages::{state::jwt, PanelContext, RightPanel},
};

#[derive(Clone, PartialEq, Debug)]
pub struct ChatState {
    id: i32,
    chat: Option<ChatInfo>,
    messages: Option<Vec<MessageInfo>>,
}

#[component]
pub fn Chat(chat_id: i32) -> Element {
    let mut state = use_signal(|| ChatState {
        id: 0,
        chat: None,
        messages: None,
    });

    use_effect({
        if state.read().id != chat_id {
            spawn(async move {
                let chat = match get_chat(jwt().await, chat_id).await {
                    Ok(chat) => Some(chat),
                    Err(err) => {
                        error!("Failed to fetch chat: {}", err);
                        None
                    }
                };
                let messages = match list_messages(jwt().await, chat_id).await {
                    Ok(msgs) => Some(msgs),
                    Err(err) => {
                        error!("Failed to fetch messages: {}", err);
                        None
                    }
                };
                state.write().id = chat_id;
                state.write().chat = chat;
                state.write().messages = messages;
            });
        }

        || {}
    });

    let state = state.read();
    if state.chat.is_none()
        || state.messages.is_none()
    {
        return rsx! { Spinner {} };
    }

    let chat = state.chat.as_ref().unwrap().clone();
    let messages = state.messages.as_ref().unwrap();

    rsx! {
        ChatHeader { chat }

        { messages.iter().map(|msg| rsx! {
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
pub fn ChatHeader(chat: ChatInfo) -> Element {
    let mut panel_state = use_context::<PanelContext>();

    rsx! {
        div {
            class: "flex chat-header p-2 border-b border-gray-300, text-center",

            IconButton {
                alt: "back",
                icon: asset!("assets/icons/back.svg"),
                onclick: move |_| {
                    panel_state.right.set(RightPanel::Empty);
                },
            }

            label {
                class: "w-full text-2xl font-bold",

                "{chat.name}"
            }
        }
    }
}
