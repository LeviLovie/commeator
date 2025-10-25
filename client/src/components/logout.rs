use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[cfg(target_arch = "wasm32")]
pub struct LogOutResponse {
    logout_url: String,
}

#[component]
pub fn LogOut() -> Element {
    rsx! {
        button {
            class: "px-4 py-2 bg-red-500 rounded mb-6",
            onclick: |_| async { logout().await },
            "Log Out"
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn logout() {
    use crate::backend::Request;
    use utils::config::{endpoints::auth::URI_LOGOUT, on_auth_base_url};

    match Request::get(&on_auth_base_url(URI_LOGOUT).await)
        .build()
        .send_decode::<LogOutResponse>()
        .await
    {
        Ok(response) => {
            navigator().replace(response.logout_url);
        }
        Err(e) => {
            error!("Error during logout request: {}", e);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn logout() {
    use crate::{Route, backend::local_storage::delete_jwt};

    delete_jwt();
    navigator().replace(Route::ViewHome);
}
