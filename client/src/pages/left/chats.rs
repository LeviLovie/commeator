use dioxus::prelude::*;
use proto::{EmptyReq, ListChatsResp};

use crate::{
    components::{Header, HeaderButton, HeaderText, Item, SmallIconButton, Spinner},
    fetch::use_fetch,
    state::AppState,
    Route,
};

#[component]
pub fn LeftChats() -> Element {
    let navigator = navigator();
    let app_state = use_context::<AppState>();
    let jwt = app_state.auth.get_jwt();

    let chats = use_fetch::<EmptyReq, ListChatsResp>("/c/list", jwt.clone(), EmptyReq {});
    if chats().loading {
        return rsx! { Spinner {} };
    }
    let chats = chats().data.as_ref().unwrap().chats.clone();

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
