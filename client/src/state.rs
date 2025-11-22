use dioxus::prelude::*;
use std::rc::Rc;

use crate::{Route, components::Spinner, services};

#[derive(Clone)]
pub struct AppState {
    pub auth: Rc<services::Auth>,
}

#[component]
pub fn AppStateLayout() -> Element {
    let auth = use_resource(async || Rc::new(services::Auth::new().run().await));

    if let Some(_) = auth.read().as_ref() {
        use_context_provider(|| AppState {
            auth: auth.read().as_ref().unwrap().clone(),
        });

        return rsx! {
            Outlet::<Route> {}
        };
    }

    rsx! {
        Spinner {}
    }
}
