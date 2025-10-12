use dioxus::prelude::*;

use crate::backend::users::check_user;

use crate::{
    Route,
    auth::get_user,
    components::Spinner,
};

#[component]
pub fn AuthCallback() -> Element {
    let user = use_resource(|| async { get_user().await });

    use_effect(move || {
        if user().is_none() || user().as_ref().unwrap().is_none() {
            return;
        }

        let user = user().as_ref().unwrap().as_ref().unwrap().clone();
        let email = user.identity.traits.email.clone();

        spawn(async move {
            match check_user(email).await {
                Ok(exists) if exists => {
                    navigator().replace(Route::Home);
                }
                Ok(_) => {
                    navigator().replace(Route::AuthProfileSetup);
                }
                Err(e) => {
                    error!("check_user failed: {e:?}");
                }
            }
        });
    });

    rsx! {
        Spinner {}
    }
}

