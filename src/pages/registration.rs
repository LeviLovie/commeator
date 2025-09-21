use dioxus::{logger::tracing::info, prelude::*};
use gloo_net::http::Request;
use serde::Deserialize;

use crate::components::CenteredForm;

#[derive(Deserialize, Debug, Clone)]
struct RegistrationFlow {
    #[serde(rename = "id")]
    _id: String,
    ui: FlowUI,
}

#[derive(Deserialize, Debug, Clone)]
struct FlowUI {
    action: String,
    method: String,
    nodes: Vec<Node>,
}

#[derive(Deserialize, Debug, Clone)]
struct Node {
    attributes: InputAttributes,
}

#[derive(Deserialize, Debug, Clone)]
struct InputAttributes {
    name: String,
    #[serde(rename = "type")]
    input_type: String,
    value: Option<String>,
}

#[component]
pub fn Registration(flow: String) -> Element {
    let flow_id = flow.clone();
    let nav = use_navigator();

    let flow = use_resource(move || {
        let flow_id = flow_id.clone();
        async move {
            let url = format!(
                "http://localhost:4433/self-service/registration/flows?id={}",
                flow_id
            );

            let res = Request::get(&url)
                .credentials(web_sys::RequestCredentials::Include)
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if res.ok() {
                res.json::<RegistrationFlow>()
                    .await
                    .map_err(|e| e.to_string())
            } else {
                Err(format!("Failed with status {}", res.status()))
            }
        }
    });

    rsx! {
        CenteredForm {
            div { class: "w-full", 
                h2 { class: "text-2xl font-bold mb-6 text-center", "Register" }

                match *flow.read() {
                    None => rsx! { p { "Loading..." } },
                    Some(Err(ref e)) => rsx! {
                        div { class: "text-red-500",
                            p { "Error loading registration flow" }
                            p { "{e}" }
                            button {
                                class: "mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                                onclick: move |_| {
                                    nav.push(crate::Route::Landing {});
                                },
                                "Back"
                            }
                        }
                    },
                    Some(Ok(ref flow)) => {
                        info!("{:?}", flow);
                        rsx! {
                            form { class: "flex flex-col gap-4",
                                action: "{flow.ui.action}",
                                method: "{flow.ui.method}",

                                {
                                    flow.ui.nodes.iter().map(|node| rsx! {
                                        div { class: "flex flex-row items-center gap-2",
                                            if node.attributes.input_type == "email" {
                                                label { class: "w-24 font-medium", "Email" }
                                            } else if node.attributes.input_type == "password" {
                                                label { class: "w-24 font-medium", "Password" }
                                            }

                                            if node.attributes.name == "method" {
                                                div {}
                                            } else {
                                                input {
                                                    class: "flex-1 border rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-400",
                                                    name: "{node.attributes.name}",
                                                    r#type: "{node.attributes.input_type}",
                                                    value: "{node.attributes.value.clone().unwrap_or_default()}",
                                                }
                                            }
                                        }
                                    })
                                }

                                button {
                                    class: "mt-4 px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600",
                                    "Sign Up"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
