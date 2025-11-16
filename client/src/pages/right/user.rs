use dioxus::prelude::*;

use proto::{GetUser, GetUserResp, VerifyPrivateChat, VerifyPrivateChatResp};
use crate::{
    components::{Avatar, Error, Header, HeaderButtonBack, HeaderText, Spinner}, request::backend, state::AppState, Route
};

#[component]
pub fn RightUser(username: String) -> Element {
    let navigator = navigator();
    let app_state = use_context::<AppState>();
    let jwt = app_state.auth.get_jwt();

    let jwt_clone = jwt.clone();
    let user = use_resource(move || {
        let jwt = jwt_clone.clone();
        let username = username.clone();
        async move {
            let req = GetUser { username: username.clone() };
            let resp: GetUserResp = backend("/u/get", jwt.clone(), req).await.expect("Failed to get user");
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
    let user_uuid = user.uuid.clone();

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
                        let jwt = jwt.clone();
                        let user_uuid = user_uuid.clone();
                        spawn(async move {
                            let req = VerifyPrivateChat {
                                with_uuid: user_uuid.clone(),
                            };
                            let resp: VerifyPrivateChatResp = backend("/c/verify", jwt, req).await.expect("Failed to verify private chat");
                            navigator.replace(Route::ViewChat { uuid: resp.chat_uuid });
                        });
                    },
                    "Message"
                }
            }
        }
    }
}
