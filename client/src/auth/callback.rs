use dioxus::prelude::*;

use crate::{backend::check_user, components::Spinner, Route};

#[component]
pub fn AuthCallback() -> Element {
    use_effect(move || {
        spawn(async move {
            match check_user().await {
                Ok(exists) if exists => {
                    navigator().replace(Route::ViewHome);
                }
                Ok(_) => {
                    navigator().replace(Route::AuthProfileSetup);
                }
                Err(_) => {
                    navigator().replace(Route::ViewHome);
                }
            }
        });
    });

    use_future(|| async {
        gloo_timers::future::TimeoutFuture::new(10_000).await;
        navigator().replace(Route::ViewHome);
    });

    rsx! {
        Spinner {}
    }
}
