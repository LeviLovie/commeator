use dioxus::prelude::*;

use crate::{backend::users::setup_user, components::CenteredForm};

#[component]
pub fn AuthProfileSetup() -> Element {
    let mut error = use_signal(|| None as Option<String>);

    let onsubmit = move |e: Event<FormData>| {
        e.prevent_default();

        spawn(async move {
            let data = e.data();

            let username = match data.get_first("username") {
                Some(v) => match v {
                    FormValue::Text(s) if !s.trim().is_empty() => s.trim().to_string(),
                    _ => {
                        error.set(Some("Invalid username".to_string()));
                        return;
                    }
                },
                None => {
                    error.set(Some("Username is required".to_string()));
                    return;
                }
            };

            let nickname = match data.get_first("nickname") {
                Some(v) => match v {
                    FormValue::Text(s) if !s.trim().is_empty() => s.trim().to_string(),
                    _ => {
                        error.set(Some("Invalid username".to_string()));
                        return;
                    }
                },
                None => {
                    error.set(Some("Username is required".to_string()));
                    return;
                }
            };

            if let Err(e) = setup_user(username, nickname).await {
                error.set(Some(format!("Failed to set up user: {}", e)));
                return;
            }

            navigator().replace(crate::Route::AuthCallback);
        });
    };

    rsx! {
        CenteredForm {
            h1 {
                class: "text-4xl font-bold text-center pb-8",
                "Profile Setup"
            }

            form {
                onsubmit: onsubmit,

                input {
                    r#type: "text",
                    name: "username",
                    placeholder: "Username",
                    class: "w-full p-3 border border-gray-300 rounded mb-4",
                    required: true,
                }

                input {
                    r#type: "text",
                    name: "nickname",
                    placeholder: "Nickname",
                    class: "w-full p-3 border border-gray-300 rounded mb-4",
                    required: true,
                }

                {
                    match error() {
                        Some(err) => rsx! {
                            div {
                                class: "bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4",
                                role: "alert",
                                "{err}"
                            }
                        },
                        None => rsx! {}
                    }
                }

                button {
                    r#type: "submit",
                    class: "w-full bg-blue-500 text-white p-3 rounded hover:bg-blue-600 transition",
                    "Save Profile"
                }
            }
        }
    }
}
