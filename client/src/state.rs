use dioxus::prelude::*;

use crate::services::{AuthService, CentrifugoService, StorageService};

#[derive(Clone)]
pub struct AppState {
    pub auth: Signal<AuthService>,
    pub storage: Signal<StorageService>,
    pub centrifugo: Signal<CentrifugoService>,
}

#[component]
pub fn AppStateLayout() -> Element {
    let default_app_state = AppState {
        auth: use_signal(AuthService::default()),
        storage: use_signal(StorageService::new()),
        centrifugo: use_signal(CentrifugoService::new()),
    };
    use_default_context_provider(default_app_state);

    rsx! {
        Outlet::<Route> {}
    }
}
