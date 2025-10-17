use dioxus::prelude::*;
use utils::requests::UserInfo;
use uuid::Uuid;

use crate::{
    backend::{list_users, my_user, new_group}, components::{Avatar, CenteredForm, IconButton, NotFullHeightSpinner}, pages::{panels::{api_data::use_api_data, right::header::Header}, PanelContext, RightPanel}
};

#[derive(Clone, PartialEq, Debug)]
pub enum Stage {
    Title,
    Users,
    Finalize,
    End,
}

#[component]
pub fn NewGroup() -> Element {
    let mut state = use_signal(|| Stage::Title);
    let title: Signal<(bool, Option<String>)> = use_signal(|| (false, None));
    let users: Signal<(bool, Vec<UserInfo>)> = use_signal(|| (false, Vec::new()));
    let finalized = use_signal(|| false);

    use_effect({
        if title.read().0 {
            state.set(Stage::Users);

            if users.read().0 {
                state.set(Stage::Finalize);

                if *finalized.read() {
                    spawn(async move {
                        info!("Creating new group...");
                        let title_guard = title.read();
                        let title: String = title_guard.1.as_ref().unwrap().clone();
                        info!("Group title: {}", title);

                        let users_guard = users.read();
                        let users = users_guard.1.clone();
                        let mut user_uuids: Vec<Uuid> = users.iter().map(|u| u.uuid).collect();
                        let my_uuid = match my_user().await {
                            Ok(user) => user.uuid,
                            Err(e) => {
                                error!("Failed to get my user: {}", e);
                                return;
                            }
                        };
                        user_uuids.push(my_uuid);

                        match new_group(title, user_uuids).await {
                            Ok(uuid) => {
                                info!("New group created with UUID: {}", uuid);
                                use_context::<PanelContext>()
                                    .right
                                    .set(RightPanel::Chat(uuid));
                            }
                            Err(e) => {
                                error!("Failed to create new group: {}", e);
                            }
                        }
                    });
                    state.set(Stage::End);
                }
            } else {
                state.set(Stage::Users);
            }
        } else {
            state.set(Stage::Title);
        }

        || {}
    });

    rsx! {
        Header { title: "New group" }

        CenteredForm {
            { match *state.read() {
                Stage::Title => rsx! { EnterTitle { title } },
                Stage::Users => rsx! { AddUsers { users } },
                Stage::Finalize => rsx! { Finalize { finalized, title, users } },
                Stage::End => rsx! { 
                    div {
                        class: "my-10",
                        NotFullHeightSpinner {}
                    }
                }
            } }
        }
    }
}

#[component]
pub fn Finalize(finalized: Signal<bool>, title: Signal<(bool, Option<String>)>, users: Signal<(bool, Vec<UserInfo>)>) -> Element {
    let users_guard = users.read();
    let users_clone = users_guard.1.clone();

    rsx! {
        p {
            class: "text-xl pb-2 text-center",
            "Verify details"
        }

        { if let Some(title) = title.read().1.as_ref() {
            rsx! {
                p {
                    class: "text-m text-center mb-4",
                    "Name: {title}"
                }
            }
        } else {
            rsx! {}
        } }

        p {
            class: "text-m text-center mb-4",
            "Members:"
        }

        { users_clone.iter().map(|user| {
            rsx! {
                UserItem { user: user.clone() }
            }
        }) }

        div {
            class: "flex flex-row mt-4",

            button {
                class: "w-full text-center text-white bg-red-600 hover:bg-red-700 rounded p-2 mr-4",
                onclick: move |_| {
                    title.write().0 = false;
                    users.write().0 = false;
                },
                "Nope"
            }

            button {
                class: "w-full text-center text-white bg-blue-600 hover:bg-blue-700 rounded p-2",
                onclick: move |_| {
                    *finalized.write() = true;
                },
                "Looks good!"
            }
        }
    }
}

#[component]
pub fn EnterTitle(title: Signal<(bool, Option<String>)>) -> Element {
    let mut local_title: Signal<String> = use_signal(|| title.read().1.clone().unwrap_or_default());

    rsx! {
        p {
            class: "text-xl text-center",
            "Give it a name"
        }

        form {
            class: "mt-2 w-full",
            onsubmit: {
                move |e| {
                    e.prevent_default();
                    match e.data().get_first("group_name").unwrap() {
                        FormValue::Text(s) if !s.trim().is_empty() => {
                            title.write().1 = Some(s.trim().to_string());
                        },
                        _ => {}
                    }
                }
            },

            input {
                class: "p-2 w-full border border-gray-300 rounded",
                placeholder: "Group name",
                r#type: "text",
                name: "group_name",
                value: "{local_title}",
                onchange: move |e| {
                    local_title.set(e.value().clone());
                },
            },

            button {
                class: "w-full text-center text-white bg-blue-600 hover:bg-blue-700 rounded p-2 mt-4",
                onclick: move |_| {
                    let local_title = local_title.read();
                    if local_title.trim().is_empty() {
                        return;
                    }
                    title.set((true, Some(local_title.trim().to_string())));
                },
                "Next"
            }
        }
    }
}

#[component]
pub fn AddUsers(users: Signal<(bool, Vec<UserInfo>)>) -> Element {
    let mut users = users;

    let all_users = use_api_data(|| async { list_users(true).await });

    {
        let all_users = all_users.read();
        if all_users.is_loading() {
            return rsx! { NotFullHeightSpinner {} };
        }
    };

    let all_users_guard = all_users.read();
    let all_users = all_users_guard.as_ref().unwrap();

    let selected_users = users.read().1.clone();

    rsx! {
        p {
            class: "text-xl text-center",
            "Add people"

            { selected_users.iter().map(|user| {
                let user = user.clone();
                rsx! {
                    div {
                        class: "flex flex-row justify-between text-left p-2 w-full h-full cursor-pointer",

                        UserItem { user: user.clone() }

                        div {
                            class: "flex flex-col justify-center mt-2",

                            IconButton {
                                alt: "Remove member".to_string(),
                                icon: asset!("/assets/icons/remove.svg"),
                                ty: "button".to_string(),
                                onclick: move |_| {
                                    users.write().1.retain(|u| u.uuid != user.uuid);
                                },
                            }
                        }
                    }
                }
            }) }

            p {
                class: "border-t border-gray-300 mt-4 pt-4 text-center text-gray-500",
                "Add more?"
            }

            { all_users.iter().map(|user| {
                if selected_users.contains(user) {
                    return rsx! {};
                }

                let user_clone = user.clone();
                let user = user.clone();
                rsx! {
                    div {
                        class: "flex flex-row justify-between text-left p-2 w-full h-full cursor-pointer",

                        UserItem { user: user.clone() }

                        div {
                            class: "flex flex-col justify-center mt-2",

                            IconButton {
                                alt: "Add member".to_string(),
                                icon: asset!("/assets/icons/add.svg"),
                                ty: "button".to_string(),
                                onclick: move |_| {
                                    users.write().1.push(user_clone.clone());
                                },
                            }
                        }
                    }
                }
            }) }

            button {
                class: "w-full text-center text-white bg-blue-600 hover:bg-blue-700 rounded p-2 mt-4",
                onclick: move |_| {
                    users.write().0 = true;
                },
                "Done"
            }
        }
    }
}

#[component]
pub fn UserItem(user: UserInfo) -> Element {
    rsx! {
        div {
            class: "flex flex-row text-left p-2 w-full h-full cursor-pointer",
            div {
                class: "flex-shrink-0 w-8 h-8 mr-3",

                Avatar { email_hash: user.email_hash.clone() },
            }

            div {
                class: "flex flex-row",

                div {
                    class: "flex flex-col justify-center mr-2",
                    p { class: "text-s", "{user.nickname}" }
                }

                div {
                    class: "flex flex-col justify-center",
                    p { class: "text-gray-500 text-s", "({user.username})" }
                }
            }
        }
    }
}
