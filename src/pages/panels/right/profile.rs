use dioxus::prelude::*;

use crate::{
    backend::{chats::verify_private_chat, users::{get_user, UserInfo}}, components::{Avatar, Spinner}, pages::{panels::right::header::Header, state::jwt, LeftPanel, PanelContext, RightPanel}
};

#[derive(Clone, PartialEq, Debug)]
pub struct ProfileState {
    username: String,
    user: Option<UserInfo>,
}

#[component]
pub fn Profile(username: String) -> Element {
    let mut state = use_signal(|| ProfileState {
        username: String::new(),
        user: None,
    });

    use_effect({
        if state.read().username != username {
            spawn(async move {
                let user = match get_user(jwt().await, username.clone()).await {
                    Ok(user) => Some(user),
                    Err(err) => {
                        error!("Failed to fetch profile: {}", err);
                        None
                    }
                };
                state.write().username = username.clone();
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
                Avatar { email: user.email.clone() },
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
                        let username = user.username.clone();
                        spawn(async move {
                            match verify_private_chat(jwt().await, username).await {
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
