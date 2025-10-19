use dioxus::prelude::*;
use futures::{SinkExt, StreamExt, lock::Mutex};
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use gloo_net::websocket::{Message, futures::WebSocket};

use crate::backend::get_centrifugo_jwt;
use utils::{config::wss_base_url, updates::Update};

#[derive(Clone)]
pub struct CentrifugoContext {
    pub client: Rc<CentrifugoClient>,
}

type Subscriber = Rc<RefCell<dyn FnMut(Update)>>;

pub struct CentrifugoClient {
    subscribers: Rc<RefCell<HashMap<String, Vec<Subscriber>>>>,
    ws: Rc<Mutex<Option<WebSocket>>>,
}

impl CentrifugoClient {
    pub fn new() -> Self {
        Self {
            subscribers: Rc::new(RefCell::new(HashMap::new())),
            ws: Rc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(&self) -> Result<()> {
        let token = get_centrifugo_jwt()
            .await
            .context("Failed to get Centrifugo JWT")?;
        let base_url = wss_base_url().await;
        let ws_url = format!(
            "{}/connection/websocket?format=json&token={}",
            base_url, token
        );

        let mut ws = WebSocket::open(&ws_url).context("Failed to open WebSocket")?;

        let connect_msg = serde_json::json!({
            "id": 1,
            "connect": { "token": token }
        });
        ws.send(Message::Text(connect_msg.to_string()))
            .await
            .context("Failed to send connect message")?;

        *self.ws.lock().await = Some(ws);

        let ws_clone = self.ws.clone();
        let subs_clone = self.subscribers.clone();

        spawn(async move {
            loop {
                let msg_opt = {
                    let mut ws_lock = ws_clone.lock().await;
                    if let Some(ws) = ws_lock.as_mut() {
                        ws.next().await
                    } else {
                        None
                    }
                };

                let msg = match msg_opt {
                    Some(msg) => msg,
                    None => break,
                };

                if let Ok(Message::Text(txt)) = msg {
                    if txt == "{}" {
                        if let Some(ws) = &mut *ws_clone.lock().await {
                            ws.send(Message::Text("{}".to_string())).await.ok();
                        }
                        continue;
                    }

                    let Ok(json) = serde_json::from_str::<Value>(&txt) else {
                        continue;
                    };
                    let Some(push) = json.get("push") else {
                        continue;
                    };
                    let channel = push
                        .get("channel")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let Some(pub_data) = push.get("pub") else {
                        continue;
                    };
                    let Some(data) = pub_data.get("data") else {
                        continue;
                    };
                    if txt == "{}" {
                        let mut ws_lock = ws_clone.lock().await;
                        if let Some(ws) = ws_lock.as_mut() {
                            ws.send(Message::Text("{}".to_string())).await.ok();
                        }
                        continue;
                    }

                    match serde_json::from_value::<Update>(data.clone()) {
                        Ok(update) => {
                            if let Some(listeners) = subs_clone.borrow().get(channel) {
                                for sub in listeners {
                                    let mut cb = sub.borrow_mut();
                                    (cb)(update.clone());
                                }
                            }
                        }
                        Err(e) => error!("Failed to parse update: {}", e),
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn subscribe<F>(&self, channel: &str, callback: F) -> anyhow::Result<()>
    where
        F: FnMut(Update) + 'static,
    {
        self.subscribers
            .borrow_mut()
            .entry(channel.to_string())
            .or_default()
            .push(Rc::new(RefCell::new(callback)));

        if let Some(ws) = &mut *self.ws.lock().await {
            let subscribe_msg = serde_json::json!({
                "id": 2,
                "subscribe": { "channel": channel }
            });
            ws.send(Message::Text(subscribe_msg.to_string()))
                .await
                .context("Failed to send subscribe message")?;
        }

        Ok(())
    }
}
