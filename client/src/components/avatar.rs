pub use dioxus::prelude::*;

#[component]
pub fn Avatar(link: String) -> Element {
    rsx! {
        img {
            class: "rounded-full",
            src: link,
            alt: "User Avatar",
        }
    }
}
