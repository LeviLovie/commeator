mod chats;
mod item;
mod settings;
mod users;

pub use chats::Chats;
pub use item::Item;
pub use settings::Settings;
pub use users::Users;

use dioxus::prelude::*;

use crate::pages::PanelContext;

#[derive(Clone)]
pub enum LeftPanel {
    Chats,
    Users,
    Settings,
}

#[component]
pub fn LeftPanelWrapper() -> Element {
    let context = use_context::<PanelContext>();
    let panel = context.left.read();
    match *panel {
        LeftPanel::Chats => rsx! { Chats {} },
        LeftPanel::Users => rsx! { Users {} },
        LeftPanel::Settings => rsx! { Settings {} },
    }
}
