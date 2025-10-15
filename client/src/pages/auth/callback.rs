use dioxus::prelude::*;

use crate::{Route, backend::check_user, components::Spinner};

#[component]
pub fn AuthCallback() -> Element {
    use_effect(move || {
        spawn(async move {
            match check_user().await {
                Ok(exists) if exists => {
                    info!("User exists, navigating to home");
                    navigator().replace(Route::Home);
                }
                Ok(_) => {
                    info!("User does not exist, navigating to profile setup");
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
