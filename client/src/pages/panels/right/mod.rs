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
use uuid::Uuid;

use crate::pages::PanelContext;

#[derive(Debug, Clone)]
pub enum RightPanel {
    Empty,
    Chat(Uuid),
    Settings,
    Profile(Uuid),
}

#[component]
pub fn RightPanelWrapper() -> Element {
    let context = use_context::<PanelContext>();
    let panel = context.right.read();

    match panel.clone() {
        RightPanel::Empty => rsx! { Empty {} },
        RightPanel::Chat(uuid) => rsx! { Chat { uuid } },
        RightPanel::Settings => rsx! { Settings {} },
        RightPanel::Profile(uuid) => rsx! { Profile { uuid } },
    }
}
