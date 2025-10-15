use dioxus::prelude::*;

use crate::{
    backend::list_users,
    components::{Avatar, Spinner},
    pages::{ApiData, Item, PanelContext, RightPanel, panels::api_data::use_api_data},
};
use utils::requests::UserInfo;

#[derive(Clone)]
pub struct UsersContext {
    users: Signal<ApiData<Vec<UserInfo>>>,
}

#[component]
pub fn Users() -> Element {
    {
        let users = use_api_data(|| async { list_users(true).await });
        let context = UsersContext { users };
        use_context_provider(|| context.clone());
    }

    let context = use_context::<UsersContext>();
    let users = context.users.read();
    if users.is_loading() {
        return rsx! { Spinner {} };
    }
    let users = users.as_ref().unwrap();

    rsx! {
        div {
            { users.iter().map(|user| {
                let user_uuid = user.uuid;
                rsx! {
                    Item {
                        div {
                            class: "flex flex-row text-left p-2 w-full h-full hover:bg-gray-300 cursor-pointer",
                            onclick: move |_| {
                                use_context::<PanelContext>().right.set(RightPanel::Profile(user_uuid));
                            },

                            div {
                                class: "flex-shrink-0 w-10 h-10 mr-3",

                                Avatar { email: user.email.clone() },
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
