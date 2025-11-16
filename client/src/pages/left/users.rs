use dioxus::prelude::*;

use crate::{
    Route,
    components::{Avatar, Header, HeaderText, Item, Spinner},
    request::backend,
    state::AppState,
};
use proto::{ListUsers, ListUsersResp};

#[component]
pub fn LeftUsers() -> Element {
    let navigator = navigator();
    let app_state = use_context::<AppState>();
    let jwt = app_state.auth.get_jwt();

    let users = use_resource(move || {
        let jwt_clone = jwt.clone();
        async move {
            let req = ListUsers {
                exclude_me: true,
            };
            let resp: ListUsersResp = backend("/u/list", jwt_clone, req).await.expect("Failed to list users");
            resp.users
        }
    });
    if users.read().is_none() {
        return rsx! { Spinner {} };
    }
    let users = users.read().as_ref().unwrap().clone();

    rsx! {
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
                            navigator.replace(Route::ViewUser { username });
                        },

                        div {
                            class: "flex-shrink-0 w-10 h-10 mr-3",

                            Avatar { link: user.avatar.clone() },
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
