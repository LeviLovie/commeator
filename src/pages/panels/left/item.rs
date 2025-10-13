use dioxus::prelude::*;

#[component]
pub fn Item(children: Element) -> Element {
    rsx! {
        div {
            class: "chat-item w-full mb-1 text-left bg-gray-200",
            { children }
        }
    }
}
