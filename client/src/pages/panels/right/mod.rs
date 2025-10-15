mod chat;
mod empty;
mod header;
mod profile;
mod settings;

pub use chat::Chat;
pub use empty::Empty;
pub use profile::Profile;
pub use settings::*;

use dioxus::prelude::*;

use crate::pages::PanelContext;

#[derive(Debug, Clone)]
pub enum RightPanel {
    Empty,
    Chat(i32),
    Settings,
    Profile(String),
}

#[component]
pub fn RightPanelWrapper() -> Element {
    let context = use_context::<PanelContext>();
    let panel = context.right.read();

    match panel.clone() {
        RightPanel::Empty => rsx! { Empty {} },
        RightPanel::Chat(chat_id) => rsx! { Chat { chat_id } },
        RightPanel::Settings => rsx! { Settings {} },
        RightPanel::Profile(username) => rsx! { Profile { username } },
    }
}
