use dioxus::prelude::*;
use proto::ListChatsResp;

use crate::{
    Route,
    components::{Header, HeaderButton, HeaderText, Item, SmallIconButton, Spinner},
    request::backend_get,
    state::AppState,
};

#[component]
pub fn LeftChats() -> Element {
    let navigator = navigator();
    let app_state = use_context::<AppState>();
    let jwt = app_state.auth.get_jwt();

    let chats = use_resource(move || {
        let jwt_clone = jwt.clone();
        async move {
            backend_get::<ListChatsResp>("/c/list", jwt_clone)
                .await
                .expect("Failed to list chats")
                .chats
        }
    });
    if chats.read().is_none() {
        return rsx! { Spinner {} };
    }
    let chats = chats.read().as_ref().unwrap().clone();

    rsx! {
        Header {
            left: rsx! {
                HeaderText { text: "Chats" }
            },
            center: rsx! {},
            right: rsx! {
                HeaderButton {
                    SmallIconButton {
                        alt: "New group".to_string(),
                        icon: asset!("/assets/icons/add.svg"),
                        ty: "button".to_string(),
                        onclick: move |_| {
                            navigator.replace(Route::ViewNewGroup);
                        },
                    }
                }
            },
        }

        { chats.iter().map(|chat| {
            let uuid = chat.uuid.clone();
            rsx! {
                Item {
                    button {
                        class: "text-left p-2 w-full h-full hover:bg-gray-300 cursor-pointer",
                        onclick: move |_| {
                            navigator.replace(Route::ViewChat { uuid: uuid.to_string() });
                        },

                        "{chat.name}"
                    }
                }
            }
        }) }
    }
}
