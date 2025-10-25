use dioxus::prelude::*;

use crate::components::Spinner;

#[component]
pub fn AuthRedirect() -> Element {
    #[cfg(target_arch = "wasm32")]
    spawn(async {
        match crate::backend::get_jwt().await {
            Some(jwt) => {
                navigator().replace(format!("commeator://callback?jwt={}", jwt));
            }
            None => {
                error!("Error retrieving JWT");
            }
        }
    });

    rsx! {
        Spinner {}
    }
}
