pub use dioxus::prelude::*;

#[component]
pub fn Avatar(email: String) -> Element {
    let email_hash = md5::compute(email.as_bytes());

    rsx! {
        img {
            class: "rounded-full",
            src: format!("https://www.gravatar.com/avatar/{:x}??s=200&d=identicon", email_hash),
            alt: "User Avatar",
        }
    }
}
