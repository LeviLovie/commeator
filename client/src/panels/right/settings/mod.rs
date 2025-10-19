mod account;

pub use account::SettingsAccount;

use dioxus::prelude::*;

use crate::pages::{Empty, PanelContext, panels::right::header::Header};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsPage {
    Empty,
    Account,
}

impl std::fmt::Display for SettingsPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsPage::Empty => write!(f, "Settings"),
            SettingsPage::Account => write!(f, "Account"),
        }
    }
}

#[component]
pub fn Settings() -> Element {
    let context = use_context::<PanelContext>();
    let page = context.settings_page.read();

    rsx! {
        Header { title: format!("{}", page) }

        match *page {
            SettingsPage::Empty => rsx! { Empty {} },
            SettingsPage::Account => rsx! { SettingsAccount {} },
        }
    }
}
