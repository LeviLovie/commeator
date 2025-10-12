use dioxus::prelude::*;

use crate::backend::chats::ChatInfo;

#[component]
pub fn Chat(chat_info: ChatInfo) -> Element {
    rsx! {
        p { "Chat {chat_info:?}" }
    }
}
