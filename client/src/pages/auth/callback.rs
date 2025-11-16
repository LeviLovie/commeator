use dioxus::prelude::*;

use crate::{Route, components::Spinner, request::backend_get};
use proto::CheckUserResp;

#[component]
pub fn AuthCallback() -> Element {
    use_effect(move || {
        spawn(async move {
            let resp: CheckUserResp = backend_get("/u/check", "").await.expect("Failed to check user");
            if resp.exists {
                navigator().replace(Route::ViewHome);
            } else {
                navigator().replace(Route::AuthProfileSetup);
            }
        });
    });

    rsx! {
        Spinner {}
    }
}
