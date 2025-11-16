use dioxus::prelude::*;

use proto::{GetUser, GetUserResp};
use crate::{
    components::{Avatar, Error, Header, HeaderButtonBack, HeaderText, Spinner}, request::backend, state::AppState, Route
};

#[component]
pub fn RightUser(username: String) -> Element {
    let app_state = use_context::<AppState>();
    let jwt = app_state.auth.get_jwt();

    let user = use_resource(move || {
        let jwt = jwt.clone();
        let username = username.clone();
        async move {
            let req = GetUser { username: username.clone() };
            let resp: GetUserResp = backend("/u/get", jwt.clone(), req).await;
            resp.user
        }
    });
    if user.read().is_none() {
        return rsx! { Spinner {} };
    }
    let user = user.read().as_ref().unwrap().clone();
    if user.is_none() {
        rsx! {
            Error { text: "User not found"  }
        };
    }
    let user = user.unwrap();

    rsx! {
        Header {
            left: rsx! { HeaderButtonBack {
                route: Route::ViewUsers {},
            } },
            center: rsx! { HeaderText {
                text: "{user.username}"
            } },
            right: rsx! {}
        }

        div {
            class: "flex flex-col items-center p-6",

            div {
                Avatar { link: user.avatar.clone() },
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
                        // let user_uuid = user.uuid;
                        spawn(async move {
                            // match verify_private_chat(user_uuid).await {
                            //     Ok(chat_uuid) => {
                            //         navigator.replace(Route::ViewChat { uuid: chat_uuid.to_string() });
                            //     }
                            //     Err(e) => {
                            //         error!("Failed to verify or create private chat: {}", e);
                            //     }
                            // }
                        });
                    },
                    "Message"
                }
            }
        }
    }
}
