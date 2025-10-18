use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    backend::{chat_users, get_chat, list_messages, my_user, send_message},
    components::{Avatar, IconButton, SmallIconButton, Spinner},
    pages::{panels::right::header::Header, CentrifugoContext, LayoutContext, PanelLayout},
};
use utils::{
    data::{ChatInfo, MessageInfo, UserInfo},
    updates::Update,
};

#[derive(Clone, PartialEq, Debug)]
pub struct ChatState {
    uuid: Option<Uuid>,
    chat: Option<ChatInfo>,
    members: Option<Vec<UserInfo>>,
    my_user: Option<UserInfo>,
    messages: Option<Vec<MessageInfo>>,
}

pub type ChatUpdatesSignal = Signal<Vec<(Uuid, Update)>>;
#[derive(Clone, Debug)]
pub struct ChatUpdatesContext(pub ChatUpdatesSignal);

#[component]
pub fn Chat(uuid: Uuid) -> Element {
    let centrifugo = use_context::<CentrifugoContext>();
    let mut state = use_signal(|| ChatState {
        uuid: None,
        chat: None,
        members: None,
        my_user: None,
        messages: None,
    });
    let context_message = use_signal::<Option<Uuid>>(|| None);

    use_effect({
        let client = centrifugo.client.clone();
        let mut updates = use_context::<ChatUpdatesContext>();
        move || {
            let client = client.clone();
            spawn(async move {
                let _ = client
                    .subscribe(&format!("chat_{}", uuid), move |update| {
                        updates.0.push((uuid, update));
                    })
                    .await;
            });
        }
    });

    use_effect({
        let updates = use_context::<ChatUpdatesContext>();
        let mut state = state;
        move || {
            let mut updates = updates.0.read().clone();
            if updates.is_empty() {
                return;
            }
            let mut state_guard = state.write();
            for (uuid, update) in updates.iter() {
                if uuid != &state_guard.uuid.unwrap_or_default() {
                    continue;
                }
                if let Update::NewMessage(message) = update
                    && state_guard.messages.is_some()
                    && !state_guard
                        .messages
                        .as_ref()
                        .unwrap()
                        .iter()
                        .any(|m| m.uuid == message.uuid)
                {
                    state_guard.messages.as_mut().unwrap().push(message.clone());
                }
            }
            updates.clear();
        }
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
                    rsx! { MessageItem { user, message: message.clone(), is_me, context_message, } }
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
pub fn MessageItem(user: Option<UserInfo>, message: MessageInfo, is_me: bool, context_message: Signal<Option<Uuid>>,) -> Element {
    let layout_signal = use_context::<LayoutContext>().layout;
    let layout_guard = layout_signal.read();
    let layout = layout_guard.clone();

    let location_right = is_me && layout == PanelLayout::Mobile;

    let container_class = if location_right {
        "flex justify-end mb-2"
    } else {
        "flex justify-start mb-2"
    };

    let bubble_color = if is_me { "bg-green-200" } else { "bg-white" };

    rsx! {
        div { class: "{container_class}",
            { if !location_right && let Some(ref user) = user { rsx! {
                MessageAvatar { email_hash: user.email_hash.clone() }
            } } else { rsx! {} } }

            div {
                class: "flex flex-col max-w-[65%] min-w-[50px]",

                div {
                    class: "{bubble_color} text-gray-900 rounded-2xl px-4 py-2 inline-flex w-full break-words shadow",

                    p {
                        class: "whitespace-pre-wrap break-words text-sm",
                        "{message.content}"
                    }

                    SmallIconButton {
                        alt: "Options".to_string(),
                        icon: asset!("/assets/icons/options.svg"),
                        ty: "button".to_string(),
                        onclick: move |_| {
                            context_message.set(Some(message.uuid));
                        },
                    }
                }

                { if context_message.read().is_some_and(|uuid| uuid == message.uuid) {
                    rsx! {
                        div {
                            class: "flex justify-center mt-1",
                            div {
                                class: "bg-yellow-100 text-yellow-800 text-xs px-2 py-1 rounded",
                                "Context Message"
                            }
                        }
                    }
                } else {
                    rsx! {}
                } }
            }

            { if location_right && let Some(ref user) = user { rsx! {
                MessageAvatar { email_hash: user.email_hash.clone() }
            } } else { rsx! {} } }
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
