use std::{cell::RefCell, rc::Rc};
use gloo_net::websocket::{futures::WebSocket, Message};
use futures::{SinkExt, StreamExt};
use dioxus::prelude::*;

use crate::backend::get_centrifugo_jwt;

pub async fn connect_to_centrifugo_channel<F>(channel: &str, on_message: F)
where
    F: 'static + FnMut(serde_json::Value),
{
    let token = get_centrifugo_jwt().await.expect("Failed to get Centrifugo JWT");

    let base_url = env!("BASE_URL_WSS").trim_end_matches('/');
    let ws_url = format!("{}/connection/websocket?format=json&token={}", base_url, token);

    let mut ws = WebSocket::open(&ws_url).expect("Failed to open WebSocket");

    let connect_msg = serde_json::json!({
        "id": 1,
        "connect": {
            "token": token,
            "subs": {
                channel: {}
            }
        }
    });
    ws.send(Message::Text(connect_msg.to_string()))
        .await
        .unwrap();

    if let Some(msg) = ws.next().await {
        match msg {
            Ok(_) => {},
            Err(e) => {
                error!("WebSocket error: {}", e);
                return;
            }
        }
    }

    let on_message = Rc::new(RefCell::new(on_message));
    while let Some(msg) = ws.next().await {
        match msg {
            Ok(Message::Text(txt)) => {
                if txt == "{}" {
                    ws.send(Message::Text("{}".to_string()))
                        .await
                        .unwrap();
                } else if let Ok(json) = serde_json::from_str::<serde_json::Value>(&txt) {
                    if let Some(json) = json.get("push") {
                        if let Some(json) = json.get("pub") {
                            if let Some(json) = json.get("data") {
                                on_message.borrow_mut()(json.clone());
                            } else {
                                error!("No data field in push pub: {}", txt);
                            }
                        } else {
                            error!("Unexpected push format: {}", txt);
                        }
                    } else {
                        error!("Unexpected message format: {}", txt);
                    }
                }
            }
            Ok(Message::Bytes(_)) => {
                warn!("Received binary message, ignoring");
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }
}
