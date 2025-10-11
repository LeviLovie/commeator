use dioxus::prelude::*;

use crate::{
    auth::get_user,
    components::{CenteredForm, LogOut, Spinner},
};

#[component]
pub fn Home() -> Element {
    let user = use_resource(|| async { get_user().await });
    if user().is_none() || user().as_ref().unwrap().is_none() {
        return rsx! { Spinner {} };
    }
    let user = user().unwrap().unwrap();

    rsx! {
        CenteredForm {
            p {
                class: "text-4xl font-bold text-gray-800 mb-4",
                "Welcome, {user.identity.traits.email}!"
            }

            LogOut {}
        }
    }
}
