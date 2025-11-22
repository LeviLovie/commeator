use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    Route,
    components::{
        Avatar, Error, Header, HeaderButtonBack, HeaderText, IconButton, NotFullHeightSpinner,
        Spinner,
    },
    fetch::use_fetch,
    pages::{LayoutContext, PanelLayout},
    request::{backend},
    services::{self, messages::Message},
    state::AppState,
};
use proto::{
    DeleteMessage, DeleteMessageResp, EditMessage, EditMessageResp, EmptyReq, GetChat, GetChatResp,
    MyUserResp, SendMessage, SendMessageResp, User,
};

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

#[component]
pub fn RightChat(uuid: String) -> Element {
    let chat_uuid = Uuid::parse_str(&uuid).expect("Invalid chat UUID");
    let app_state = use_context::<AppState>();
    let jwt = app_state.auth.get_jwt();

    let my_user = use_fetch::<EmptyReq, MyUserResp>("/u/my", jwt.clone(), EmptyReq {});
    let chat = use_fetch::<GetChat, GetChatResp>("/c/get", jwt.clone(), GetChat { uuid: uuid.clone() });
    if my_user().loading || chat().loading {
        return rsx! { Spinner {} };
    }

    if chat().data.is_none() || my_user().data.is_none() {
        return rsx! {
            Error { text: "Failed to load"  }
        };
    }

    let my_user = my_user().data.as_ref().unwrap().clone().user.unwrap();
    let members = chat().data.as_ref().unwrap().members.clone();
    let chat = chat().data.as_ref().unwrap().chat.clone().unwrap();

    let default_interaction = use_signal(|| Interaction::None);
    use_context_provider(|| default_interaction);
    let mut interaction = use_context::<Signal<Interaction>>();
    use_effect(move || {
        interaction.set(Interaction::None);
    });

    let chat_uuid_clone = chat_uuid.clone();
    let mut last_chat_uuid = use_signal(|| Uuid::nil());
    let mut chat_messsages: Signal<Option<Vec<Message>>> = use_signal(|| None);
    let jwt_clone = jwt.clone();
    spawn({
        let jwt = jwt_clone.clone();
        let chat_uuid = chat_uuid_clone.clone();
        async move {
            if *last_chat_uuid.read() == chat_uuid {
                return;
            }

            last_chat_uuid.set(chat_uuid.clone());

            let messages = services::Messages::new()
                .load(&jwt, chat_uuid)
                .await
                .expect("Failed to get messages");
            chat_messsages.set(Some(messages.messages));
        }
    });

    rsx! {
        div {
            class: "flex flex-col h-full",

            Header {
                left: rsx! { HeaderButtonBack {
                    route: Route::ViewChats {},
                } },
                center: rsx! { HeaderText {
                    text: "{chat.name}"
                } },
                right: rsx! {}
            }

            div {
                class: "flex-1 overflow-y-auto p-4 space-y-2 bg-gray-50",
                id: "message-container",

                { if let Some(chat_messages) = &chat_messsages.read().as_ref() {
                    let my_user = members.iter()
                        .find(|u| u.username == my_user.username)
                        .cloned()
                        .expect("Failed to find my user in chat members");
                    let jwt = jwt.clone();
                    rsx! {
                        { chat_messages.iter().map(|message| {
                            message_item(jwt.clone(), &members, &my_user, message.clone())
                        }) }
                    }
                } else {
                    rsx! {
                        NotFullHeightSpinner {}
                    }
                } }
            }

            MessageBox { chat_uuid: uuid.clone(), jwt: jwt.clone(), uuid: chat_uuid }
        }
    }
}

pub fn message_item(jwt: String, users: &[User], my_user: &User, message: Message) -> Element {
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
        { if let Some(reply) = &message.reply_to { rsx! {
            div {
                class: "flex flex-row {reply_margin} {container_class}",
                style: "margin-bottom: -15px;",

                MessageBubble {
                    jwt: jwt.clone(),
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
                    jwt: jwt.clone(),
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

                    Interaction::Delete { uuid: context_uuid } if context_uuid == message.uuid => {
                        let jwt = jwt.clone();
                        rsx! {
                            button {
                                class: "bg-red-200 hover:bg-red-300 text-sm px-4 py-2 rounded-2xl mr-2",
                                onclick: move |e| {
                                    e.prevent_default();
                                    interaction.set(Interaction::None);
                                    spawn({
                                        let message_uuid = message.uuid;
                                        let jwt = jwt.clone();
                                        async move {
                                            backend::<DeleteMessage, DeleteMessageResp>(
                                                "/m/delete",
                                                jwt.clone(),
                                                DeleteMessage {
                                                    uuid: message_uuid.to_string(),
                                                },
                                            ).await.expect("Failed to delete message");
                                        }
                                    });
                                },

                                "Sure? :("
                            }
                        }
                    }

                    _ => { rsx! {} }
                } }
            }

            { if location_right { rsx! {
                MessageBubble {
                    jwt: jwt.clone(),
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
    jwt: String,
    uuid: Uuid,
    content: String,
    sender: Option<User>,
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
                link: sender.avatar.clone(),
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
                link: sender.avatar.clone(),
                tint: is_reply,
            }
        } } else { rsx! {} } }
    }
}

#[component]
pub fn MessageAvatar(link: String, tint: bool) -> Element {
    let tint = if tint { "brightness-75" } else { "" };

    rsx! {
        div {
            class: "flex items-end mr-2 w-9 h-9 ml-2 {tint}",
            Avatar { link }
        }
    }
}

#[component]
pub fn MessageBox(chat_uuid: String, jwt: String, uuid: Uuid) -> Element {
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
                                let jwt = jwt.clone();
                                async move {
                                    backend::<EditMessage, EditMessageResp>(
                                        "/m/edit",
                                        jwt.clone(),
                                        EditMessage {
                                            uuid: edit_uuid.to_string(),
                                            new_content: new_content.clone(),
                                        },
                                    ).await.expect("Failed to delete message");
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
                            let jwt = jwt.clone();
                            let chat_uuid = chat_uuid.clone();
                            spawn(async move {
                                backend::<SendMessage, SendMessageResp>(
                                    "/m/send",
                                    jwt.clone(),
                                    SendMessage {
                                        chat_uuid: chat_uuid.to_string(),
                                        content: msg.clone(),
                                        reply_to: Some(reply.to_string()),
                                    },
                                ).await.expect("Failed to delete message");
                            });
                        }

                        _ => {
                            let msg = message.read().trim().to_string();
                            if msg.is_empty() {
                                return;
                            }
                            message.set(String::new());
                            let jwt = jwt.clone();
                            let chat_uuid = chat_uuid.clone();
                            spawn(async move {
                                backend::<SendMessage, SendMessageResp>(
                                    "/m/send",
                                    jwt.clone(),
                                    SendMessage {
                                        chat_uuid: chat_uuid.to_string(),
                                        content: msg.clone(),
                                        reply_to: None,
                                    },
                                ).await.expect("Failed to delete message");
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
