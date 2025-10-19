use dioxus::prelude::*;

use crate::{
    Route,
    backend::{get_username, verify_private_chat},
    components::{Avatar, Header, HeaderButton, HeaderText, IconButton, Spinner},
};
use utils::data::UserInfo;

#[derive(Clone, PartialEq, Debug)]
pub struct UserState {
    username: Option<String>,
    user: Option<UserInfo>,
}

#[component]
pub fn RightUser(username: String) -> Element {
    let navigator = navigator();
    let mut state = use_signal(|| UserState {
        username: None,
        user: None,
    });

    use_effect({
        let update = if let Some(ref current) = state.read().username {
            *current != username
        } else {
            true
        };

        if update {
            spawn(async move {
                let user = match get_username(username.clone()).await {
                    Ok(user) => Some(user),
                    Err(err) => {
                        error!("Failed to fetch profile: {}", err);
                        None
                    }
                };
                state.write().username = Some(username);
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
        div {
            class: "flex flex-col h-screen",

            Header {
                left: rsx! { HeaderButton {
                    IconButton {
                        alt: "back",
                        ty: "button",
                        icon: asset!("assets/icons/back.svg"),
                        onclick: move |_| {
                            navigator.go_back();
                        },
                    }
                } },
                center: rsx! { HeaderText {
                    text: "{user.username}"
                } },
                right: rsx! {}
            }

            div {
                class: "flex flex-col items-center p-6",

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
                                    Ok(chat_uuid) => {
                                        navigator.push(Route::ViewChat { uuid: chat_uuid.to_string() });
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
}
