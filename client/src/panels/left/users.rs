use dioxus::prelude::*;

use crate::{
    Route,
    backend::{ApiData, list_users, use_api_data},
    components::{Avatar, Header, HeaderText, Item, Spinner},
};
use utils::data::UserInfo;

#[derive(Clone)]
pub struct UsersContext {
    users: Signal<ApiData<Vec<UserInfo>>>,
}

#[component]
pub fn LeftUsers() -> Element {
    {
        let users = use_api_data(|| async { list_users(true).await });
        let context = UsersContext { users };
        use_context_provider(|| context.clone());
    }

    let navigator = navigator();

    let context = use_context::<UsersContext>();
    let users = context.users.read();
    if users.is_loading() || users.as_ref().is_none() {
        return rsx! { Spinner {} };
    }
    let users = users.as_ref().unwrap();

    rsx! {
        div {
            Header {
                left: rsx! {
                    HeaderText { text: "Users" }
                },
                center: rsx! {},
                right: rsx! {},
            }

            { users.iter().map(|user| {
                let username = user.username.clone();
                rsx! {
                    Item {
                        div {
                            class: "flex flex-row text-left p-2 w-full h-full hover:bg-gray-300 cursor-pointer",
                            onclick: move |_| {
                                let username = username.clone();
                                navigator.push(Route::ViewUser { username });
                            },

                            div {
                                class: "flex-shrink-0 w-10 h-10 mr-3",

                                Avatar { email_hash: user.email_hash.clone() },
                            }

                            div {
                                class: "flex flex-col",
                                p { class: "m-0 p-0 text-s", "{user.nickname}" }
                                p { class: "m-0 p-0 text-gray-500 text-xs", "@{user.username}" }
                            }
                        }
                    }
                }
            }) }
        }
    }
}
