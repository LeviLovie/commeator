use dioxus::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct LogOutResponse {
    logout_url: String,
}

#[component]
pub fn LogOut() -> Element {
    rsx! {
        button {
            class: "px-4 py-2 bg-red-500 rounded mb-6",
            onclick: |_| async {
                match Request::get("http://localhost:4433/self-service/logout/browser")
                    .credentials(web_sys::RequestCredentials::Include)
                    .send()
                    .await {
                        Ok(response) if response.ok() => {
                            match response.json::<LogOutResponse>().await {
                                Ok(response) => {
                                    navigator().replace(response.logout_url);
                                }
                                Err(e) => {
                                    error!("Error parsing logout response: {}", e);
                                }
                            }
                        },
                        Ok(response) => {
                            error!("Received non-OK response during logout: {}", response.status());
                        },
                        Err(e) => {
                            error!("Error during logout request: {}", e);
                        },
                    }
            },
            "Log Out"
        }
    }
}
