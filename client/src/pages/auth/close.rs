use dioxus::prelude::*;

use crate::components::{CenteredForm, CenteredText};

#[component]
pub fn AuthClose() -> Element {
    rsx! {
        CenteredForm {
            CenteredText {
                text: "You can now close this window."
            }
        }
    }
}
