use dioxus::prelude::*;

use crate::{
    backend::users::{get_user, UserInfo}, components::{Avatar, Spinner}, pages::{panels::right::header::Header, state::jwt}
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
            class: "p-4",

            div {
                class: "flex flex-col items-center space-y-4",

                Avatar { email: user.email.clone() },
            }

            p {
                class: "text-4xl font-bold text-center",
                {user.nickname.clone()}
            }

            p {
                class: "text-s text-center",
                "@{user.username}"
            }
        }
    }
}
