use dioxus::prelude::*;
use utils::requests::UserInfo;
use uuid::Uuid;

use crate::{
    backend::{get_user, verify_private_chat},
    components::{Avatar, Spinner},
    pages::{LeftPanel, PanelContext, RightPanel, panels::right::header::Header},
};

#[derive(Clone, PartialEq, Debug)]
pub struct ProfileState {
    uuid: Option<Uuid>,
    user: Option<UserInfo>,
}

#[component]
pub fn Profile(uuid: Uuid) -> Element {
    let mut state = use_signal(|| ProfileState {
        uuid: None,
        user: None,
    });

    use_effect({
        let update = if let Some(current_uuid) = state.read().uuid {
            current_uuid != uuid
        } else {
            true
        };

        if update {
            spawn(async move {
                let user = match get_user(uuid).await {
                    Ok(user) => Some(user),
                    Err(err) => {
                        error!("Failed to fetch profile: {}", err);
                        None
                    }
                };
                state.write().uuid = Some(uuid);
                state.write().user = user;
            });
        }

        || {}
    });

    let state = state.read();
    if state.user.is_none() {
        return rsx! { Spinner {} };
    }

    let user = state.user.as_ref().unwrap().clone();

    rsx! {
        Header { title: "{user.nickname}" }

        div {
            class: "p-4 w-full flex flex-col items-center",

            div {
                Avatar { email_hash: user.email_hash.clone() },
            }

            div {
                class: "mb-4",

                p {
                    class: "text-4xl font-bold",
                    {user.nickname.clone()}
                }

                p {
                    class: "text-s",
                    "@{user.username}"
                }
            }

            div {
                button {
                    class: "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2",
                    onclick: move |_| {
                        let user_uuid = user.uuid;
                        spawn(async move {
                            match verify_private_chat(user_uuid).await {
                                Ok(chat_id) => {
                                    let mut context = use_context::<PanelContext>();
                                    context.left.set(LeftPanel::Chats);
                                    context.right.set(RightPanel::Chat(chat_id));
                                }
                                Err(e) => {
                                    error!("Failed to verify or create private chat: {}", e);
                                }
                            }
                        });
                    },
                    "Message"
                }
            }
        }
    }
}
