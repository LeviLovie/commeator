// #[macro_export]
// macro_rules! verify_user_jwt {
//     () => {{
//         let user = use_resource(|| async { $crate::backend::get_kratos_user().await });
//         if user().is_none() || user().as_ref().unwrap().is_none() {
//             return rsx! { $crate::components::Spinner {} };
//         }
//         let user = user().as_ref().unwrap().as_ref().unwrap().clone();
//
//         user
//     }};
// }
