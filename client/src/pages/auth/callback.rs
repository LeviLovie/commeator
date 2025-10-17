use dioxus::prelude::*;

use crate::{backend::{check_user, get_kratos_user}, components::Spinner, Route};

#[component]
pub fn AuthCallback() -> Element {
    use_effect(move || {
        spawn(async move {
            match check_user().await {
                Ok(exists) if exists => {
                    navigator().replace(Route::Home);
                }
                Ok(_) => {
                    navigator().replace(Route::AuthProfileSetup);
                }
                Err(e) => {
                    error!("Error checking user existence: {}", e);
                    navigator().replace(Route::Home);
                }
            }
        });
    });

    use_future(|| async {
        gloo_timers::future::TimeoutFuture::new(10_000).await;
        navigator().replace(Route::Home);
    });

    rsx! {
        Spinner {}
    }
}
