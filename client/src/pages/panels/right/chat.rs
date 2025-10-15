use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    backend::{get_chat, list_messages, send_message},
    components::{IconButton, Spinner},
    pages::panels::right::header::Header,
};
use utils::requests::{ChatInfo, MessageInfo};

#[derive(Clone, PartialEq, Debug)]
pub struct ChatState {
    uuid: Option<Uuid>,
    chat: Option<ChatInfo>,
    messages: Option<Vec<MessageInfo>>,
}

#[component]
pub fn Chat(uuid: Uuid) -> Element {
    let mut state = use_signal(|| ChatState {
        uuid: None,
        chat: None,
        messages: None,
    });

    use_effect({
        let update = if let Some(current_uuid) = state.read().uuid {
            current_uuid != uuid
        } else {
            true
        };

        if update {
            spawn(async move {
                let chat = match get_chat(uuid).await {
                    Ok(chat) => Some(chat),
                    Err(err) => {
                        error!("Failed to fetch chat: {}", err);
                        None
                    }
                };
                let messages = match list_messages(uuid).await {
                    Ok(msgs) => Some(msgs),
                    Err(err) => {
                        error!("Failed to fetch messages: {}", err);
                        None
                    }
                };
                state.write().uuid = Some(uuid);
                state.write().chat = chat;
                state.write().messages = messages;
            });
        }

        || {}
    });

    let state = state.read();
    if state.chat.is_none() || state.messages.is_none() {
        return rsx! { Spinner {} };
    }

    let chat = state.chat.as_ref().unwrap().clone();
    let messages = state.messages.as_ref().unwrap();

    rsx! {
        div {
            class: "flex flex-col h-screen",

            div {
                class: "flex-none",
                Header { title: "{chat.name}" }
            }

            div {
                class: "flex-1 overflow-y-auto p-4 space-y-2 bg-gray-50",
                id: "message-container",

                { messages.iter().map(|message| {
                    rsx! { MessageItem { message: message.clone() } }
                }) }
            }

            div {
                class: "border-t border-gray-300 bg-white p-2 sticky bottom-0",
                MessageBox { uuid }
            }
        }
    }
}

#[component]
pub fn MessageItem(message: MessageInfo) -> Element {
    rsx! {
        div {
            class: "p-2 border-b border-gray-300",

            div {
                class: "font-bold",
                "User ID: {message.sender_nickname}"
            }

            div {
                "{message.content}"
            }

            div {
                class: "text-sm text-gray-500",

                "Sent at: {message.created_at}"
            }
        }
    }
}

#[component]
pub fn MessageBox(uuid: Uuid) -> Element {
    let mut message = use_signal(String::new);

    rsx! {
        form {
            class: "flex gap-2",
            onsubmit: move |e| {
                e.prevent_default();

                let msg = message.read().trim().to_string();
                if msg.is_empty() {
                    return;
                }

                spawn(async move {
                    if let Err(e) = send_message(uuid, msg).await {
                        error!("Failed to send message: {}", e);
                    }
                    message.set(String::new());
                });
            },

            input {
                class: "flex-1 p-2 border border-gray-300 rounded",
                placeholder: "Type your message...",
                value: "{message}",
                oninput: move |e| {e.prevent_default(); message.set(e.value().clone())},
            },

            IconButton {
                alt: "Send".to_string(),
                icon: asset!("/assets/icons/forward.svg"),
                ty: "submit".to_string(),
            }
        }
    }
}
