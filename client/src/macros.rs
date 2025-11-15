#[macro_export]
macro_rules! can_fail {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                dioxus::prelude::error!("{err:?}");
                return rsx! {
                    $crate::components::Error {
                        text: format!("An error occurred: {err}"),
                    }
                };
            }
        }
    };
}
