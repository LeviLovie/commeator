use dioxus::prelude::*;

use crate::{
    Route,
    components::{Header, HeaderButtonBack, HeaderText, LogOut, Spinner},
    request::backend_get,
    state::AppState,
};
use proto::GetUserResp;

#[component]
pub fn SettingsAccount() -> Element {
    let app_state = use_context::<AppState>();
    let jwt = app_state.auth.get_jwt();

    let user = use_resource(move || {
        let jwt = jwt.clone();
        async move {
            backend_get::<GetUserResp>("/u/my", jwt.clone())
                .await
                .expect("Failed to get self")
                .user
        }
    });
    if user.read().is_none() {
        return rsx! { Spinner {} };
    }
    let user = user.read().as_ref().unwrap().clone().unwrap();

    rsx! {
        Header {
            left: rsx! { HeaderButtonBack {
                route: Route::ViewSettings {},
            } },
            center: rsx! { HeaderText {
                text: "Account"
            } },
            right: rsx! {}
        }

        div {
            class: "flex flex-col p-4 space-y-4",

            p {
                class: "text-s",
                "{user.nickname} @{user.username}"
            }

            LogOut {}
        }
    }
}
