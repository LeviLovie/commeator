#[macro_export]
macro_rules! verify_uuid {
    ($uuid:expr) => {{
        match uuid::Uuid::parse_str(&$uuid) {
            Ok(parsed) => parsed,
            Err(_) => {
                return rsx! {
                    $crate::components::Error {
                        text: "Invalid UUID"
                    }
                };
            }
        }
    }};
}
