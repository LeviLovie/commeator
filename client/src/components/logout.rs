use dioxus::prelude::*;
use serde::Deserialize;

use crate::backend::Request;
use utils::config::auth::URL_LOGOUT;

#[derive(Deserialize, Debug, Clone)]
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

pub async fn logout() {
    match Request::get(URL_LOGOUT)
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
