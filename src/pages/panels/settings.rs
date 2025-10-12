use dioxus::prelude::*;

use crate::{backend::users::get_my_user, components::{LogOut, Spinner}, pages::{state::jwt, PanelContext}, verify_user_jwt};

#[component]
pub fn Settings() -> Element {
    let mut context = use_context::<PanelContext>();

    use_effect(move || {
        let user = context.user.read().clone();
        if !user.0 && user.1.is_none() {
            context.user.set((true, None));
            spawn(async move {
                match get_my_user(jwt().await).await {
                    Ok(u) => {
                        context.user.set((true, Some(u)));
                    }
                    Err(e) => {
                        error!("Failed to fetch chats: {e:?}");
                    },
                }
            });
        }
    });

    let user = context.user.read();

    if !user.0 || user.1.is_none() {
        return rsx! {
            Spinner {}
        };
    }

    let user = user.1.clone().unwrap();

    rsx! {
        div {
            div {
                class: "text-center mt-2 mb-4",

                label {
                    class: "text-4xl font-bold",
                    "Settings"
                }
            }

            div {
                class: "flex flex-col space-y-2 text-center",

                label {
                    class: "text-2xl",
                    "Account"
                }

                label {
                    class: "text-s",
                    "{user.email}"
                }

                label {
                    class: "text-s",
                    "{user.nickname} ({user.nickname})"
                }

                LogOut {}
            }
        }
    }
}
