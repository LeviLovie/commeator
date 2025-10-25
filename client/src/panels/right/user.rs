use dioxus::prelude::*;

use crate::{
    Route,
    backend::{get_username, verify_private_chat},
    components::{Avatar, Header, HeaderButtonBack, HeaderText, Spinner},
};
use utils::{data::UserInfo, LogError};

#[derive(Clone, PartialEq, Debug)]
pub enum UserState {
    Uninitialized,
    Loading,
    Loaded {
        username: String,
        user: UserInfo,
    },
}

#[component]
pub fn RightUser(username: String) -> Element {
    let navigator = navigator();
    let mut state = use_signal(|| UserState::Uninitialized);

    use_effect({
        if match state.read().clone() {
            UserState::Uninitialized => true,
            UserState::Loading => false,
            UserState::Loaded {
                username: current_username,
                ..
            } => current_username != username,
        } {
            *state.write() = UserState::Loading;

            spawn(async move {
                let user = get_username(username.clone()).await.log_error().expect("Failed to fetch user profile");

                *state.write() = UserState::Loaded {
                    username,
                    user,
                }
            });
        }

        || {}
    });

    match state.read().clone() {
        UserState::Uninitialized | UserState::Loading => {
            rsx! { Spinner {} }
        }
        UserState::Loaded {
            user,
            ..
        } => { rsx! {
            Header {
                left: rsx! { HeaderButtonBack {
                    route: Route::ViewUsers,
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
                                        navigator.replace(Route::ViewChat { uuid: chat_uuid.to_string() });
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
        } }
    }
}
