pub use dioxus::prelude::*;

#[component]
pub fn Avatar(email_hash: String) -> Element {
    rsx! {
        img {
            class: "rounded-full",
            src: format!("https://www.gravatar.com/avatar/{}??s=200&d=identicon", email_hash),
            alt: "User Avatar",
        }
    }
}
