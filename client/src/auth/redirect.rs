use dioxus::prelude::*;

#[component]
#[cfg(target_arch = "wasm32")]
pub fn AuthRedirect(id: String) -> Element {
    use crate::{components::Spinner, verify_uuid};

    let key = verify_uuid!(&id);

    spawn(async move {
        if let Err(e) = crate::backend::natives_authenticate(key.clone()).await {
            error!("Error during native authentication: {}", e);
        }
        info!("Native authentication successful");
        navigator().replace(crate::Route::AuthClose);
    });

    rsx! {
        Spinner {}
    }
}

#[component]
#[cfg(not(target_arch = "wasm32"))]
pub fn AuthRedirect(id: String) -> Element {
    use crate::components::{CenteredForm, CenteredText};

    rsx! {
        CenteredForm {
            CenteredText {
                text: "This page is only meant to be used from web."
            }

            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    navigator().replace(crate::Route::ViewHome);
                },
                "Well... Go home..."
            }
        }
    }
}
