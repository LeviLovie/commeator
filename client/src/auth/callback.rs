use dioxus::prelude::*;
use utils::sleep_ms;

use crate::{Route, backend::check_user, components::Spinner};

#[component]
pub fn AuthCallback() -> Element {
    use_effect(move || {
        spawn(async move {
            match check_user().await {
                Ok(exists) if exists => {
                    error!("[AuthCallback] User exists, redirecting to home");
                    navigator().replace(Route::ViewHome);
                }
                Ok(_) => {
                    error!("[AuthCallback] User does not exist, redirecting to profile setup");
                    navigator().replace(Route::AuthProfileSetup);
                }
                Err(e) => {
                    error!(
                        "[AuthCallback] Error checking user existence, redirecting to home: {}",
                        e
                    );
                    navigator().replace(Route::ViewHome);
                }
            }
        });
    });

    use_future(|| async {
        sleep_ms(10_000).await;
        navigator().replace(Route::ViewHome);
    });

    rsx! {
        Spinner {}
    }
}
