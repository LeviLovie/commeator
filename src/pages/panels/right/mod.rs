mod chat;
mod empty;
mod settings;

pub use chat::Chat;
pub use empty::Empty;
pub use settings::*;

use dioxus::prelude::*;

use crate::pages::PanelContext;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RightPanel {
    Empty,
    Chat(i32),
    Settings,
}

#[component]
pub fn RightPanelWrapper() -> Element {
    let context = use_context::<PanelContext>();
    let panel = context.right.read();

    match *panel {
        RightPanel::Empty => rsx! { Empty {} },
        RightPanel::Chat(chat_id) => rsx! { Chat { chat_id } },
        RightPanel::Settings => rsx! { Settings {} },
    }
}
