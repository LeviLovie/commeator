use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    backend::{chat_users, get_chat, list_messages, my_user, send_message}, centrifugo::connect_to_centrifugo_channel, components::{Avatar, IconButton, Spinner}, pages::panels::right::header::Header
};
use utils::requests::{ChatInfo, MessageInfo, UserInfo};

#[derive(Clone, PartialEq, Debug)]
pub struct ChatState {
    uuid: Option<Uuid>,
    chat: Option<ChatInfo>,
    members: Option<Vec<UserInfo>>,
    my_user: Option<UserInfo>,
    messages: Option<Vec<MessageInfo>>,
}

#[component]
pub fn Chat(uuid: Uuid) -> Element {
    let mut state = use_signal(|| ChatState {
        uuid: None,
        chat: None,
        members: None,
        my_user: None,
        messages: None,
    });

    spawn(async move {
        connect_to_centrifugo_channel(
            format!("chat_{}", uuid).as_str(),
            move |json| {
                let message = serde_json::from_value::<MessageInfo>(json.clone());
                if let Ok(message) = message && state.read().messages.is_some() {
                    if state.read().messages.as_ref().unwrap().iter().any(|m| m.uuid == message.uuid) {
                        return;
                    }
                    state.write().messages.as_mut().unwrap().push(message);
                } else {
                    error!("Failed to parse incoming message: {:?}", json);
                }
            },
        ).await;
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
                let members = match chat_users(uuid).await {
                    Ok(users) => Some(users),
                    Err(err) => {
                        error!("Failed to fetch chat users: {}", err);
                        None
                    }
                };
                let my_user = match my_user().await {
                    Ok(user) => Some(user),
                    Err(err) => {
                        error!("Failed to fetch my user: {}", err);
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
                state.write().members = members;
                state.write().my_user = my_user;
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
                    let user = state.members.as_ref().unwrap().iter().find(|u| u.uuid == message.sender_uuid).cloned();
                    let is_me = if let Some(my_user) = &state.my_user {
                        my_user.uuid == message.sender_uuid
                    } else {
                        false
                    };
                    rsx! { MessageItem { user, message: message.clone(), is_me } }
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
pub fn MessageItem(user: Option<UserInfo>, message: MessageInfo, is_me: bool) -> Element {
    let container_class = if is_me {
        "flex justify-end mb-2"
    } else {
        "flex justify-start mb-2"
    };

    let bubble_class = if is_me {
        "bg-green-200 text-gray-900 rounded-2xl rounded px-4 py-2 max-w-[65%] shadow"
    } else {
        "bg-white text-gray-900 rounded-2xl rounded px-4 py-2 max-w-[65%] shadow"
    };

    rsx! {
        div { class: "{container_class}",
            { if !is_me && let Some(ref user) = user {
                rsx! { MessageAvatar { email_hash: user.email_hash.clone() } }
            } else { rsx! {} } }

            div { class: "{bubble_class}",
                p { class: "whitespace-pre-wrap break-words text-sm", "{message.content}" }
            }

            { if is_me && let Some(ref user) = user {
                rsx! { MessageAvatar { email_hash: user.email_hash.clone() } }
            } else { rsx! {} } }
        }
    }
}

#[component]
pub fn MessageAvatar(email_hash: String) -> Element {
    rsx! {
        div {
            class: "flex items-end mr-2 w-9 h-9 ml-2",
            Avatar { email_hash, }
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
