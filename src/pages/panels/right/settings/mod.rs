mod account;

pub use account::SettingsAccount;

use dioxus::prelude::*;

use crate::pages::{Empty, PanelContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsPage {
    Empty,
    Account,
}

#[component]
pub fn Settings() -> Element {
    let context = use_context::<PanelContext>();
    let page = context.settings_page.read();
    info!("Rendering settings page: {:?}", *page);

    match *page {
        SettingsPage::Empty => rsx! { Empty {} },
        SettingsPage::Account => rsx! { SettingsAccount {} },
    }
}
