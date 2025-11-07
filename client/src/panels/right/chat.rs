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
    LogError,
    data::{ChatInfo, MessageInfo, UserInfo},
    sleep_ms,
    updates::Update,
};

#[derive(Clone, PartialEq, Debug)]
pub enum ChatState {
    Uninitialized,
    Loading,
    Loaded {
        uuid: Uuid,
        my_user: UserInfo,
        chat: ChatInfo,
        members: Vec<UserInfo>,
        messages: Vec<MessageInfo>,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub enum Interaction {
    None,
    Selected {
        uuid: Uuid,
    },
    Delete {
        uuid: Uuid,
    },
    Reply {
        uuid: Uuid,
        content: String,
    },
    Edit {
        uuid: Uuid,
        content: String,
        copy_content: bool,
    },
}

pub static CHAT_UPDATES: GlobalSignal<Vec<(Uuid, Update)>> = GlobalSignal::new(Vec::new);

#[component]
pub fn RightChat(uuid: String) -> Element {
    let uuid = verify_uuid!(uuid);
    let centrifugo = use_context::<CentrifugoContext>();
    let mut state = use_signal(|| ChatState::Uninitialized);
    {
        let default_interaction = use_signal(|| Interaction::None);
        use_context_provider(|| default_interaction);
    }

    use_effect({
        if match state.read().clone() {
            ChatState::Uninitialized => true,
            ChatState::Loading => false,
            ChatState::Loaded {
                uuid: current_uuid, ..
            } => current_uuid != uuid,
        } {
            *state.write() = ChatState::Loading;

            spawn(async move {
                let (chat_res, members_res, my_user_res, messages_res) = futures::join!(
                    get_chat(uuid),
                    chat_users(uuid),
                    my_user(),
                    list_messages(uuid),
                );

                *state.write() = ChatState::Loaded {
                    uuid,
                    chat: chat_res.log_error().expect("Failed to fetch chat"),
                    members: members_res.log_error().expect("Failed to fetch chat users"),
                    my_user: my_user_res.log_error().expect("Failed to fetch my user"),
                    messages: messages_res.log_error().expect("Failed to fetch messages"),
                };
            });
        }

        || {}
    });

    spawn(async move {
        centrifugo
            .client
            .subscribe(&format!("chat_{}", uuid), move |update| {
                CHAT_UPDATES.write().push((uuid, update));
            })
            .await
            .log_error()
            .expect("Failed to subscribe to chat updates");
    });

    spawn(async move {
        loop {
            sleep_ms(100).await;

            let updates = CHAT_UPDATES.read().clone();
            if !updates.is_empty() {
                CHAT_UPDATES.write().clear();

                match &mut *state.write() {
                    ChatState::Uninitialized | ChatState::Loading => continue,
                    ChatState::Loaded { messages, .. } => {
                        for (_, update) in updates.iter() {
                            match update {
                                Update::NewMessage(message) => {
                                    if !messages.iter().any(|m| m.uuid == message.uuid) {
                                        messages.push(message.clone());
                                    }
                                }
                                Update::DeleteMessage(payload) => {
                                    messages.retain(|m| m.uuid != payload.message_uuid);
                                }
                                Update::UpdateMessage(payload) => {
                                    if let Some(message) =
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
            }
        }
    });

    match &*state.read() {
        ChatState::Uninitialized | ChatState::Loading => {
            rsx! { Spinner {} }
        }
        ChatState::Loaded {
            uuid,
            my_user,
            chat,
            members,
            messages,
            ..
        } => {
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
                            message_item(members, my_user, message.clone())
                        }) }
                    }

                    MessageBox { uuid: *uuid }
                }
            }
        }
    }
}

pub fn message_item(users: &[UserInfo], my_user: &UserInfo, message: MessageInfo) -> Element {
    let layout_signal = use_context::<LayoutContext>().layout;
    let layout_guard = layout_signal.read();
    let layout = layout_guard.clone();
    let mut interaction = use_context::<Signal<Interaction>>();

    let user = users
        .iter()
        .find(|u| u.uuid == message.sender_uuid)
        .cloned();
    let is_me = my_user.uuid == message.sender_uuid;

    let location_right = is_me && layout == PanelLayout::Mobile;

    let container_class = if location_right {
        "flex justify-end mb-2"
    } else {
        "flex justify-start mb-2"
    };

    let reply_margin = if location_right { "mr-5" } else { "ml-5" };

    rsx! {
        { if let Some(reply) = &message.reply { rsx! {
            div {
                class: "flex flex-row {reply_margin} {container_class}",
                style: "margin-bottom: -15px;",

                MessageBubble {
                    uuid: reply.uuid,
                    content: reply.content.clone(),
                    sender: users.iter().find(|u| u.uuid == reply.sender_uuid).cloned(),
                    is_me: my_user.uuid == reply.sender_uuid,
                    is_reply: true,
                    location_right,
                    edited: reply.edited_at.is_some(),
                }
            } }
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

            div {
                class: "flex flex-row justify-start ml-2",

                { match interaction.read().clone() {
                    Interaction::Selected { uuid: context_uuid } if context_uuid == message.uuid => {
                        let message_clone_reply = message.clone();
                        let message_clone_edit = message.clone();
                        let message_clone_delete = message.clone();

                        rsx! {
                            button {
                                class: "bg-blue-200 hover:bg-blue-300 text-sm px-4 py-2 rounded-2xl mr-2",
                                onclick: move |e| {
                                    e.prevent_default();
                                    interaction.set(Interaction::Reply {
                                        uuid: message_clone_reply.uuid,
                                        content: message_clone_reply.content.clone(),
                                    });
                                },

                                "Reply"
                            }

                            { if is_me { rsx! {
                                button {
                                    class: "bg-yellow-200 hover:bg-yellow-300 text-sm px-4 py-2 rounded-2xl mr-2",
                                    onclick: move |e| {
                                        e.prevent_default();
                                        interaction.set(Interaction::Edit {
                                            uuid: message_clone_edit.uuid,
                                            content: message_clone_edit.content.clone(),
                                            copy_content: true,
                                        });
                                    },

                                    "Edit"
                                }

                                button {
                                    class: "bg-red-200 hover:bg-red-300 text-sm px-4 py-2 rounded-2xl mr-2",
                                    onclick: move |e| {
                                        e.prevent_default();
                                        interaction.set(Interaction::Delete {
                                            uuid: message_clone_delete.uuid,
                                        });
                                    },

                                    "Delete"
                                }
                            } } else { rsx! {} }
                        }
                    } }

                    Interaction::Delete { uuid: context_uuid } if context_uuid == message.uuid => { rsx! {
                        button {
                            class: "bg-red-200 hover:bg-red-300 text-sm px-4 py-2 rounded-2xl mr-2",
                            onclick: move |e| {
                                e.prevent_default();
                                interaction.set(Interaction::None);
                                spawn({
                                    let message_uuid = message.uuid;
                                    async move {
                                        if let Err(e) =  delete_message(message_uuid).await {
                                            error!("Failed to delete message {}: {}", message_uuid, e);
                                        }
                                    }
                                });
                            },

                            "Sure? :("
                        }
                    } }

                    _ => { rsx! {} }
                } }
            }

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
    let mut interaction = use_context::<Signal<Interaction>>();

    let bubble_color = match *interaction.read() {
        Interaction::Reply {
            uuid: context_uuid, ..
        } if context_uuid == uuid => {
            if is_me {
                "bg-green-400"
            } else {
                "bg-gray-400"
            }
        }
        Interaction::Edit {
            uuid: context_uuid, ..
        } if context_uuid == uuid => "bg-yellow-200",
        Interaction::Delete { uuid: context_uuid } if context_uuid == uuid => "bg-red-200",
        _ => {
            if is_reply {
                if is_me { "bg-green-400" } else { "bg-gray-400" }
            } else if is_me {
                "bg-green-200"
            } else {
                "bg-white"
            }
        }
    };

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

                        let mut interaction = interaction.write();
                        if matches!(*interaction, Interaction::Selected { uuid: current_uuid } if current_uuid == uuid) {
                            *interaction = Interaction::None;
                        } else {
                            *interaction = Interaction::Selected { uuid };
                        }
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
    let mut interaction = use_context::<Signal<Interaction>>();
    let mut message = use_signal(String::new);

    let icon = match *interaction.read() {
        Interaction::Edit { .. } => asset!("/assets/icons/edit.svg"),
        _ => asset!("/assets/icons/forward.svg"),
    };

    use_effect({
        let interaction_state = interaction.read().clone();
        if let Interaction::Edit {
            uuid,
            content,
            copy_content,
        } = interaction_state
            && copy_content
        {
            if message.read().is_empty() {
                message.set(content.clone());
            }

            interaction.set(Interaction::Edit {
                uuid,
                content: content.clone(),
                copy_content: false,
            });
        }

        || {}
    });

    rsx! {
        div {
            class: "flex flex-col sticky bottom-0 bg-white border-t border-gray-300 p-2",

            { match interaction.read().clone() {
                Interaction::Reply { content, .. } => {
                    let mut content = content.clone();
                    content.truncate(30);

                    rsx! {
                        div {
                            class: "bg-gray-200 p-1 mb-2 rounded flex justify-between items-center",
                            p {
                                "Replying to message: {content}"
                            }
                        }
                    }
                }

                Interaction::Edit { content, .. } => {
                    let mut content = content.clone();
                    content.truncate(30);

                    rsx! {
                        div {
                            class: "bg-yellow-200 p-1 mb-2 rounded flex justify-between items-center",

                            p {
                                "Editing message: {content}"
                            }
                        }
                    }
                }

                _ => { rsx! {} }
            } }

            form {
                class: "flex gap-2",
                onsubmit: move |e| {
                    e.prevent_default();
                    let message_clone = message.read().clone();
                    let message_clone = message_clone.trim().to_string();
                    if message_clone.is_empty() {
                        return;
                    }

                    match &mut *interaction.write() {
                        Interaction::Edit { uuid: edit_uuid, .. } => {
                            let new_content = message_clone.trim().to_string();
                            if new_content.is_empty() {
                                return;
                            }
                            message.set(String::new());
                            spawn({
                                let edit_uuid = *edit_uuid;
                                async move {
                                    if let Err(e) = edit_message(edit_uuid, new_content).await {
                                        error!("Failed to edit message: {}", e);
                                    }
                                }
                            });
                        },

                        Interaction::Reply { uuid: reply_uuid, .. } => {
                            let msg = message.read().trim().to_string();
                            if msg.is_empty() {
                                return;
                            }
                            message.set(String::new());
                            let reply = *reply_uuid;
                            spawn(async move {
                                if let Err(e) = send_message(uuid, msg, Some(reply)).await {
                                    error!("Failed to send message: {}", e);
                                }
                            });
                        }

                        _ => {
                            let msg = message.read().trim().to_string();
                            if msg.is_empty() {
                                return;
                            }
                            message.set(String::new());
                            spawn(async move {
                                if let Err(e) = send_message(uuid, msg, None).await {
                                    error!("Failed to send message: {}", e);
                                }
                            });
                        }
                    }

                    *interaction.write() = Interaction::None;
                },

                { match interaction.read().clone() {
                    Interaction::Reply { .. } => { rsx! {
                        IconButton {
                            alt: "Close".to_string(),
                            ty: "button".to_string(),
                            icon: asset!("/assets/icons/close.svg"),
                            onclick: move |_| {
                                interaction.set(Interaction::None);
                            },
                        }
                    } }

                    Interaction::Edit { .. } => { rsx! {
                        div {
                            IconButton {
                                alt: "Close".to_string(),
                                ty: "button".to_string(),
                                icon: asset!("/assets/icons/close.svg"),
                                onclick: move |_| {
                                    interaction.set(Interaction::None);
                                },
                            }
                        }
                    } }

                    _ => { rsx! {} }
                } }

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
            }
        }
    }
}
