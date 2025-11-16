use dioxus::prelude::*;
use proto::{SetupUser, SetupUserResp, SetupUserResult};

use crate::{components::CenteredForm, request::backend, state::AppState};

#[component]
pub fn AuthProfileSetup() -> Element {
    let error = use_signal(|| None as Option<String>);

    let app_state = use_context::<AppState>();
    let jwt = app_state.auth.get_jwt();

    let onsubmit = move |e: Event<FormData>| {
        e.prevent_default();

        let mut error = error.clone();
        let jwt = jwt.clone();

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

            let req = SetupUser {
                username,
                nickname,
                avatar: String::new(),
            };
            let resp: SetupUserResp = backend("/u/setup", jwt.clone(), req).await.expect("Failed to setup user");
            match resp.result.try_into().unwrap() {
                SetupUserResult::Success => {
                    navigator().replace(crate::Route::AuthCallback);
                }
                SetupUserResult::UsernameTaken => {
                    error.set(Some("Username is already taken".to_string()));
                }
                SetupUserResult::InvalidUsername => {
                    error.set(Some("Username is invalid. It should be from 3 to 20 chars, only .-_ and lowercase ASCII is allowed.".to_string()));
                }
                SetupUserResult::InvalidNickname => {
                    error.set(Some(
                        "Username is invalid. It should be from 3 to 20 chars.".to_string(),
                    ));
                }
            }
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
