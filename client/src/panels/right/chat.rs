use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    Route,
    backend::{
        chat_users, delete_message, edit_message, get_chat, list_messages, my_user, send_message,
    },
    centrifugo::CentrifugoContext,
    components::{Avatar, Header, HeaderButtonBack, HeaderText, IconButton, Spinner},
    panels::{LayoutContext, PanelLayout},
    verify_uuid,
};
use utils::{
    data::{ChatInfo, MessageInfo, UserInfo},
    updates::Update,
};

#[derive(Clone, PartialEq, Debug)]
pub struct ChatState {
    is_loading: bool,
    uuid: Option<Uuid>,
    chat: Option<ChatInfo>,
    members: Option<Vec<UserInfo>>,
    my_user: Option<UserInfo>,
    messages: Option<Vec<MessageInfo>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ChatInterContext {
    message: Signal<String>,
    message_context: Signal<Option<Uuid>>,
    delete: Signal<Option<Uuid>>,
    reply: Signal<Option<Uuid>>,
    edit: Signal<(bool, Option<Uuid>)>,
}

pub static CHAT_UPDATES: GlobalSignal<Vec<(Uuid, Update)>> = GlobalSignal::new(Vec::new);

#[component]
pub fn RightChat(uuid: String) -> Element {
    let uuid = verify_uuid!(uuid);
    let centrifugo = use_context::<CentrifugoContext>();

    let mut state = use_signal(|| ChatState {
        is_loading: false,
        uuid: None,
        chat: None,
        members: None,
        my_user: None,
        messages: None,
    });

    {
        let default_message = use_signal(String::new);
        let default_message_context = use_signal(|| None);
        let default_delete = use_signal(|| None);
        let default_reply = use_signal(|| None);
        let default_edit = use_signal(|| (false, None));
        use_context_provider(|| ChatInterContext {
            message: default_message,
            message_context: default_message_context,
            delete: default_delete,
            reply: default_reply,
            edit: default_edit,
        });
    };
    let mut context = use_context::<ChatInterContext>();

    spawn(async move {
        let client = centrifugo.client.clone();
        if let Err(e) = client
            .subscribe(&format!("chat_{}", uuid), move |update| {
                CHAT_UPDATES.write().push((uuid, update));
            })
            .await
        {
            error!("Failed to subscribe to chat updates: {}", e);
        }
    });

    spawn(async move {
        loop {
            gloo_timers::future::TimeoutFuture::new(100).await;

            let updates = CHAT_UPDATES.read().clone();
            if !updates.is_empty() {
                CHAT_UPDATES.write().clear();

                let mut state_guard = state.write();
                for (update_uuid, update) in updates.iter() {
                    if update_uuid != &state_guard.uuid.unwrap_or_default() {
                        continue;
                    }

                    match update {
                        Update::NewMessage(message) => {
                            if let Some(messages) = state_guard.messages.as_mut()
                                && !messages.iter().any(|m| m.uuid == message.uuid)
                            {
                                messages.push(message.clone());
                            }
                        }

                        Update::DeleteMessage(payload) => {
                            if let Some(messages) = state_guard.messages.as_mut() {
                                messages.retain(|m| m.uuid != payload.message_uuid);
                            }
                        }

                        Update::UpdateMessage(payload) => {
                            if let Some(messages) = state_guard.messages.as_mut()
                                && let Some(message) =
                                    messages.iter_mut().find(|m| m.uuid == payload.uuid)
                            {
                                message.content = payload.new_content.clone();
                                message.edited_at = Some(payload.edited_at);
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
    });

    use_effect({
        let update = {
            let state = state.read();

            if state.is_loading {
                false
            } else if let Some(current_uuid) = state.uuid {
                info!("Current UUID: {}, New UUID: {}", current_uuid, uuid);
                current_uuid != uuid
            } else {
                info!("No current UUID, loading new chat {}", uuid);
                true
            }
        };
        info!("Update state: {}", update);

        if update {
            state.write().is_loading = true;
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

                let mut state_mut = state.write();
                state_mut.is_loading = false;
                state_mut.uuid = Some(uuid);
                state_mut.chat = chat;
                state_mut.members = members;
                state_mut.my_user = my_user;
                state_mut.messages = messages;
            });
        }

        || {}
    });

    use_effect({
        if let Some(message_uuid) = context.edit.read().1
            && !context.edit.read().0
        {
            let state = state.read();
            if let Some(messages) = &state.messages
                && let Some(message) = messages.iter().find(|m| m.uuid == message_uuid)
                && context.message.read().is_empty()
            {
                context.message.set(message.content.clone());
            }
        }

        || {}
    });

    if state.read().is_loading {
        return rsx! { Spinner {} };
    }

    let state = state.read();
    let chat = state.chat.as_ref().unwrap();
    let messages = state.messages.as_ref().unwrap();

    rsx! {
        div {
            class: "flex flex-col h-full",

            Header {
                left: rsx! { HeaderButtonBack {
                    route: Route::ViewChats,
                } },
                center: rsx! { HeaderText {
                    text: "{chat.name}"
                } },
                right: rsx! {}
            }

            div {
                class: "flex-1 overflow-y-auto p-4 space-y-2 bg-gray-50",
                id: "message-container",

                { messages.iter().map(|message| {
                    rsx! { MessageItem {
                        users: state.members.clone(),
                        my_user: state.my_user.clone(),
                        message: message.clone()
                    } }
                }) }
            }

            MessageBox { uuid }
        }
    }
}

#[component]
pub fn MessageItem(
    users: Option<Vec<UserInfo>>,
    my_user: Option<UserInfo>,
    message: MessageInfo,
) -> Element {
    let layout_signal = use_context::<LayoutContext>().layout;
    let layout_guard = layout_signal.read();
    let layout = layout_guard.clone();

    let mut context = use_context::<ChatInterContext>();

    let user = users
        .as_ref()
        .unwrap()
        .iter()
        .find(|u| u.uuid == message.sender_uuid)
        .cloned();
    let is_me = if let Some(my_user) = &my_user {
        my_user.uuid == message.sender_uuid
    } else {
        false
    };

    let location_right = is_me && layout == PanelLayout::Mobile;

    let container_class = if location_right {
        "flex justify-end mb-2"
    } else {
        "flex justify-start mb-2"
    };

    let reply_margin = if location_right { "mr-5" } else { "ml-5" };

    rsx! {
        { if let Some(reply) = message.reply {
            let is_me = if let Some(my_user) = &my_user {
                my_user.uuid == reply.sender_uuid
            } else {
                false
            };

            rsx! {
                div {
                    class: "flex flex-row {reply_margin} {container_class}",
                    style: "margin-bottom: -15px;",

                    MessageBubble {
                        uuid: reply.uuid,
                        content: reply.content.clone(),
                        sender: users.as_ref().unwrap().iter().find(|u| u.uuid == reply.sender_uuid).cloned(),
                        is_me,
                        is_reply: true,
                        location_right,
                        edited: reply.edited_at.is_some(),
                    }
                }
            }
        } else { rsx! {} } }

        div {
            class: "flex flex-row {container_class}",

            { if !location_right { rsx! {
                MessageBubble {
                    uuid: message.uuid,
                    content: message.content.clone(),
                    sender: user.clone(),
                    is_me,
                    is_reply: false,
                    location_right,
                    edited: message.edited_at.is_some(),
                }
            } } else { rsx! {} } }

            { if context.message_context.read().is_some_and(|uuid| uuid == message.uuid) {
                rsx! {
                    div {
                        class: "flex flex-row justify-start ml-2",

                        button {
                            class: "bg-blue-200 hover:bg-blue-300 text-sm px-4 py-2 rounded-2xl mr-2",
                            onclick: move |e| {
                                e.prevent_default();
                                context.message_context.set(None);
                                context.reply.set(Some(message.uuid));
                                context.delete.set(None);
                                context.edit.set((false, None));
                            },

                            "Reply"
                        }

                        { if is_me { rsx! {
                            button {
                                class: "bg-yellow-200 hover:bg-yellow-300 text-sm px-4 py-2 rounded-2xl mr-2",
                                onclick: move |e| {
                                    e.prevent_default();
                                    context.message_context.set(None);
                                    context.reply.set(None);
                                    context.delete.set(None);
                                    context.edit.set((false, Some(message.uuid)));
                                },

                                "Edit"
                            }

                            { if let Some(delete) = *context.delete.read() { rsx! {
                                button {
                                    class: "bg-red-200 hover:bg-red-300 text-sm px-4 py-2 rounded-2xl mr-2",
                                    onclick: move |e| {
                                        e.prevent_default();
                                        if delete == message.uuid {
                                            spawn({
                                                let message_uuid = message.uuid;
                                                async move {
                                                    if let Err(e) =  delete_message(message_uuid).await {
                                                        error!("Failed to delete message {}: {}", message_uuid, e);
                                                    }
                                                }
                                            });
                                        }
                                        context.message_context.set(None);
                                        context.reply.set(None);
                                        context.delete.set(None);
                                        context.edit.set((false, None));
                                    },

                                    "Sure? :("
                                }
                            } } else { rsx! {
                                button {
                                    class: "bg-red-200 hover:bg-red-300 text-sm px-4 py-2 rounded-2xl mr-2",
                                    onclick: move |e| {
                                        e.prevent_default();
                                        context.delete.set(Some(message.uuid));
                                        context.reply.set(None);
                                        context.edit.set((false, None));
                                    },

                                    "Delete"
                                }
                            } } }
                        } } else { rsx! {} } }
                    }
                }
            } else { rsx! {} } }

            { if location_right { rsx! {
                MessageBubble {
                    uuid: message.uuid,
                    content: message.content.clone(),
                    sender: user.clone(),
                    is_me,
                    is_reply: false,
                    location_right,
                    edited: message.edited_at.is_some(),
                }
            } } else { rsx! {} } }
        }
    }
}

#[component]
pub fn MessageBubble(
    uuid: Uuid,
    content: String,
    sender: Option<UserInfo>,
    is_me: bool,
    is_reply: bool,
    location_right: bool,
    edited: bool,
) -> Element {
    let mut context = use_context::<ChatInterContext>();

    let mut bubble_color = if is_reply {
        if is_me { "bg-green-400" } else { "bg-gray-400" }
    } else if is_me {
        "bg-green-200"
    } else {
        "bg-white"
    };

    if context
        .reply
        .read()
        .is_some_and(|reply_uuid| reply_uuid == uuid)
    {
        bubble_color = "bg-blue-200";
    }

    if context
        .edit
        .read()
        .1
        .is_some_and(|edit_uuid| edit_uuid == uuid)
    {
        bubble_color = "bg-yellow-200";
    }

    rsx! {
        { if !location_right && let Some(ref sender) = sender { rsx! {
            MessageAvatar {
                email_hash: sender.email_hash.clone(),
                tint: is_reply,
            }
        } } else { rsx! {} } }

        div {
            class: "flex flex-row max-w-[65%] min-w-[50px]",

            div {
                class: "inline-flex max-w-full",

                button {
                    class: "{bubble_color} px-4 py-2 text-gray-900 rounded-2xl inline-flex break-words shadow max-w-full",
                    onclick: move |_| {
                        if is_reply {
                            return;
                        }

                        if context.message_context.read().is_some_and(|context_uuid| context_uuid == uuid) {
                            context.message_context.set(None);
                            return;
                        }
                        context.message_context.set(Some(uuid));
                    },

                    { if edited { rsx! {
                        img {
                            class: "w-3 h-3 mb-1 mr-1 self-end",
                            src: asset!("/assets/icons/edit.svg"),
                            alt: "Edited",
                        }
                    } } else { rsx! {} } }

                    p {
                        class: "whitespace-pre-wrap break-words text-sm",
                        "{content}"
                    }
                }
            }
        }

        { if location_right && let Some(ref sender) = sender { rsx! {
            MessageAvatar {
                email_hash: sender.email_hash.clone(),
                tint: is_reply,
            }
        } } else { rsx! {} } }
    }
}

#[component]
pub fn MessageAvatar(email_hash: String, tint: bool) -> Element {
    let tint = if tint { "brightness-75" } else { "" };
    rsx! {
        div {
            class: "flex items-end mr-2 w-9 h-9 ml-2 {tint}",
            Avatar { email_hash }
        }
    }
}

#[component]
pub fn MessageBox(uuid: Uuid) -> Element {
    let mut context = use_context::<ChatInterContext>();
    let mut message = context.message;

    let icon = if context.edit.read().1.is_some() {
        asset!("/assets/icons/edit.svg")
    } else {
        asset!("/assets/icons/forward.svg")
    };

    rsx! {
        div {
            class: "sticky bottom-0 bg-white border-t border-gray-300 p-2",

            form {
                class: "flex gap-2",
                onsubmit: move |e| {
                    e.prevent_default();

                    let msg = message.read().trim().to_string();
                    if msg.is_empty() {
                        return;
                    }

                    if context.edit.read().1.is_some() {
                        let edit_uuid = context.edit.read().1.unwrap();
                        context.edit.set((false, None));
                        spawn(async move {
                            if let Err(e) = edit_message(edit_uuid, msg).await {
                                error!("Failed to edit message: {}", e);
                            }
                        });
                        message.set(String::new());
                    } else {
                        spawn(async move {
                            let reply = *context.reply.read();
                            if let Err(e) = send_message(uuid, msg, reply).await {
                                error!("Failed to send message: {}", e);
                            }
                            message.set(String::new());
                            context.reply.set(None);
                        });
                    }
                },

                input {
                    class: "flex-1 px-2 border border-gray-300 rounded",
                    placeholder: "Type your message...",
                    value: "{message}",
                    oninput: move |e| {e.prevent_default(); message.set(e.value().clone())},
                },

                IconButton {
                    alt: "Send".to_string(),
                    icon,
                    ty: "submit".to_string(),
                }

                { if context.reply.read().is_some() || context.edit.read().1.is_some() { rsx! {
                    IconButton {
                        alt: "Close".to_string(),
                        ty: "button".to_string(),
                        icon: asset!("/assets/icons/close.svg"),
                        onclick: move |_| {
                            context.reply.set(None);
                            context.edit.set((false, None));
                        },
                    }
                } } else { rsx! {} } }
            }
        }
    }
}
