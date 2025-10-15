use anyhow::{Result, anyhow, bail};
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use std::sync::{LazyLock, Mutex};
use utils::{
    auth::KratosUserData,
    config::{
        auth::{URL_LOGIN, URL_WHOAMI},
        endpoints::*,
        server_url,
    },
    requests::*,
};
use uuid::Uuid;

use crate::components::logout;

static JWT: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

pub enum Method {
    Get,
    Post,
}

pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl Request {
    pub fn get(url: &str) -> RequestBuilder {
        RequestBuilder {
            url: url.to_string(),
            method: Method::Get,
            headers: vec![],
            body: None,
        }
    }

    pub fn post(url: &str) -> RequestBuilder {
        RequestBuilder {
            url: url.to_string(),
            method: Method::Post,
            headers: vec![],
            body: None,
        }
    }

    #[cfg(feature = "web")]
    pub async fn send_decode<T>(self) -> Result<T>
    where
        T: DeserializeOwned + Clone,
    {
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
        let response = request.send().await?;

        if !response.ok() {
            bail!("Request failed with status: {}", response.status())
        }

        let text = response.text().await?;
        serde_json::from_str(&text).map_err(|e| anyhow!(e.to_string()))
    }

    #[cfg(not(feature = "web"))]
    pub async fn send_decode<T>(self) -> Result<T>
    where
        T: DeserializeOwned + Clone,
    {
        use reqwest::Client;

        let client = Client::new();
        let mut request = match self.method {
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
        let response = request.send().await.map_err(|e| e.to_string())?;
        if !response.status().is_success() {
            bail!("Request failed with status: {}", response.status())
        }

        let text = response.text().await.map_err(|e| e.to_string())?;
        serde_json::from_str(&text).map_err(|e| e.to_string())?
    }
}

pub struct RequestBuilder {
    pub url: String,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl RequestBuilder {
    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn add_body_from_json<T: serde::Serialize>(mut self, body: &T) -> Self {
        self.body = Some(serde_json::to_string(body).unwrap());
        self.add_header("Content-Type", "application/json")
    }

    pub async fn add_jwt(self) -> Self {
        if JWT.lock().unwrap().is_none() {
            match generate_jwt().await {
                Ok(token) => {
                    *JWT.lock().unwrap() = Some(token);
                }
                Err(e) => {
                    if e.to_string().contains("User not found") {
                        logout().await;
                    }
                    error!("Failed to request JWT: {}", e);
                }
            }
        }

        if let Some(token) = JWT.lock().unwrap().as_ref() {
            let mut headers = self.headers.clone();
            headers.push(("Authorization".to_string(), format!("Bearer {}", token)));
            return Self {
                url: self.url,
                method: self.method,
                body: self.body,
                headers,
            };
        }

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

pub async fn generate_jwt() -> Result<String> {
    let response = Request::get(&server_url(jwt::G_GENERATE))
        .build()
        .send_decode::<GenerateJwtResponse>()
        .await?;
    Ok(response.0)
}

#[allow(dead_code)]
pub async fn verify_jwt() -> Result<bool> {
    let response = Request::get(&server_url(jwt::G_VERIFY))
        .add_jwt()
        .await
        .build()
        .send_decode::<VerifyJwtResponse>()
        .await?;
    Ok(response.0)
}

pub async fn list_chats() -> Result<Vec<ChatInfo>> {
    let response = Request::get(&server_url(chats::G_LIST))
        .add_jwt()
        .await
        .build()
        .send_decode::<ListChatsResponse>()
        .await?;
    Ok(response.0)
}

pub async fn get_chat(uuid: Uuid) -> Result<ChatInfo> {
    let request = GetChatRequest(uuid);
    let response = Request::post(&server_url(chats::P_GET))
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<GetChatResponse>()
        .await?;
    Ok(response.0)
}

pub async fn verify_private_chat(user_uuid: Uuid) -> Result<Uuid> {
    let request = VerifyPrivateChatRequest {
        with_user: user_uuid,
    };
    let response = Request::post(&server_url(chats::P_VERIFY_PRIVATE))
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<VerifyPrivateChatResponse>()
        .await?;
    Ok(response.0)
}

pub async fn list_messages(chat_uuid: Uuid) -> Result<Vec<MessageInfo>> {
    let request = ListMessagesRequest(chat_uuid);
    let response = Request::post(&server_url(messages::P_LIST))
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<ListMessagesResponse>()
        .await?;
    Ok(response.0)
}

pub async fn send_message(chat_uuid: Uuid, content: String) -> Result<()> {
    let request = SendMessageRequest { chat_uuid, content };
    let _ = Request::post(&server_url(messages::P_SEND))
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<SendMessageResponse>()
        .await?;
    Ok(())
}

pub async fn check_user() -> Result<bool> {
    let response = Request::get(&server_url(users::G_CHECK))
        .build()
        .send_decode::<CheckUserResponse>()
        .await?;
    Ok(response.0)
}

pub async fn my_user() -> Result<UserInfo> {
    let response = Request::get(&server_url(users::G_ME))
        .add_jwt()
        .await
        .build()
        .send_decode::<GetUserResponse>()
        .await?;
    Ok(response.0)
}

pub async fn get_user(uuid: Uuid) -> Result<UserInfo> {
    let request = GetUserRequest(uuid);
    let response = Request::post(&server_url(users::P_GET))
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<GetUserResponse>()
        .await?;
    Ok(response.0)
}

pub async fn setup_user(username: String, nickname: String) -> Result<()> {
    let request = SetupUserRequest { username, nickname };
    let _ = Request::post(&server_url(users::P_SETUP))
        .add_body_from_json(&request)
        .build()
        .send_decode::<SetupUserResponse>()
        .await?;
    Ok(())
}

pub async fn list_users(exclude_self: bool) -> Result<Vec<UserInfo>> {
    let request = ListUsersRequest { exclude_self };
    let response = Request::post(&server_url(users::P_LIST))
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<ListUsersResponse>()
        .await?;
    Ok(response.0)
}

async fn try_get_kratos_user() -> Result<KratosUserData> {
    Request::get(URL_WHOAMI)
        .build()
        .send_decode::<KratosUserData>()
        .await
}

pub async fn get_kratos_user() -> Option<KratosUserData> {
    match try_get_kratos_user().await {
        Ok(user) => Some(user),
        Err(e) => {
            error!("Error fetching user info: {}", e);
            navigator().replace(URL_LOGIN);
            None
        }
    }
}
