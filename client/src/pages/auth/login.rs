use dioxus::prelude::*;
use serde::Deserialize;
use utils::config::endpoints::auth::url_login_flow;

use crate::{
    backend::Request,
    components::{CenteredForm, Spinner},
};

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
pub fn AuthLogIn(flow: String) -> Element {
    let flow_id = flow.clone();
    let flow = use_resource(move || {
        let flow_id = flow_id.clone();
        async move {
            match Request::get(&url_login_flow(&flow_id).await)
                .build()
                .send_decode::<RegistrationFlow>()
                .await
            {
                Ok(flow) => Some(Ok(flow)),
                Err(err) => Some(Err(err)),
            }
        }
    });

    rsx! {
        CenteredForm {
            match *flow.value().read() {
                None => rsx! { Spinner {} },
                Some(None) => rsx! { Spinner {} },
                Some(Some(Err(_))) => rsx! { Spinner {} },
                Some(Some(Ok(ref flow))) => render_flow(flow),
            }
        }
    }
}

fn render_flow(flow: &RegistrationFlow) -> Element {
    rsx! {
        h2 {
            class: "text-4xl font-bold mb-6 text-center",
            "Login"
        }

        form {
            class: "flex flex-col gap-4",
            action: "{flow.ui.action}",
            method: "{flow.ui.method}",

            {
                flow.ui.nodes.iter().map(|node| rsx! {
                    div {
                        class: "flex flex-row items-center gap-2",

                        if node.attributes.input_type == "email" {
                            label {
                                class: "w-24 font-medium",
                                "Email"
                            }
                        } else if node.attributes.input_type == "password" {
                            label {
                                class: "w-24 font-medium",
                                "Password"
                            }
                        }

                        if node.attributes.name == "method" {
                            div {}
                        } else {
                            input {
                                class: "w-full text-gray-900 border focus:ring-4 focus:outline-none font-medium rounded-lg text-m px-10 py-2.5 text-center me-2 mb-2 border-gray-600 text-gray-400 hover:text-white hover:bg-gray-600 focus:ring-gray-800",
                                name: "{node.attributes.name}",
                                r#type: "{node.attributes.input_type}",
                                value: "{node.attributes.value.clone().unwrap_or_default()}",
                            }
                        }
                    }
                })
            }

        }
    }
}
