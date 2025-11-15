use anyhow::{Result, anyhow};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub enum Method {
    Get,
    Post,
}

#[derive(Debug)]
pub struct Response {
    status: u16,
    body: Vec<u8>,
}

impl Response {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn raw(&self) -> &[u8] {
        &self.body
    }

    pub fn text(&self) -> Result<String> {
        String::from_utf8(self.body.clone()).map_err(|e| anyhow!(e.to_string()))
    }

    pub fn json<T: DeserializeOwned + Clone>(&self) -> Result<T> {
        let text = self.text()?;
        serde_json::from_str(&text).map_err(|e| anyhow!(e.to_string()))
    }
}

pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn get(url: impl Into<String>) -> RequestBuilder {
        RequestBuilder {
            url: url.into(),
            method: Method::Get,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn post(url: impl Into<String>) -> RequestBuilder {
        RequestBuilder {
            url: url.into(),
            method: Method::Post,
            headers: HashMap::new(),
            body: None,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn send(self) -> Result<Response> {
        use gloo_net::http::Request as GlooRequest;

        let mut request = match self.method {
            Method::Get => GlooRequest::get(&self.url),
            Method::Post => GlooRequest::post(&self.url),
        };
        request = request.credentials(web_sys::RequestCredentials::Include);
        let request = self
            .headers
            .iter()
            .fold(request, |req, (k, v)| req.header(k, v));
        let request = if let Some(body) = self.body {
            request.body(body)
        } else {
            request.build()
        }?;
        
        match request.send().await {
            Ok(response) => {
                let status = response.status();
                let body = response.binary().await?;
                Ok(Response {
                    status,
                    body,
                })
            }
            Err(e) => {
                // TODO: Fix to return an actual error
                Ok(Response {
                    status: 401,
                    body: vec![],
                })
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn send(self) -> Result<Response> {
        use reqwest::Client;

        let client = Client::new();
        let request = match self.method {
            Method::Get => client.get(&self.url),
            Method::Post => client.post(&self.url),
        };
        let request = self
            .headers
            .iter()
            .fold(request, |req, (k, v)| req.header(k, v));
        let request = if let Some(body) = self.body {
            request.body(body)
        } else {
            request
        };
        let response = request.send().await.map_err(|e| anyhow!(e.to_string()))?;

        let status = response.status();
        let body = response.bytes().await.map_err(|e| anyhow!(e.to_string()))?;

        Ok(Response {
            status: status.as_u16(),
            body: body.to_vec(),
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn send_block(self, callback: impl FnOnce(Result<Response>) + 'static) {
        wasm_bindgen_futures::spawn_local(async move {
            let res = self.send().await;
            callback(res);
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn send_block(self, callback: impl FnOnce(Result<Response>) + 'static) {
        callback(futures::executor::block_on(self.send()));
    }
}

pub struct RequestBuilder {
    pub url: String,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl RequestBuilder {
    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn add_json_body<T: serde::Serialize>(mut self, body: &T) -> Self {
        self.body = Some(serde_json::to_string(body).unwrap());
        self.add_header("Content-Type", "application/json")
    }

    pub async fn add_jwt(mut self, token: String) -> Self {
        self.headers
            .insert("Authorization".to_string(), format!("Bearer {}", token));
        self
    }

    pub fn build(self) -> Request {
        Request {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }
}

// TODO: Add tests
