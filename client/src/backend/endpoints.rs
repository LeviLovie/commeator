use anyhow::{Result, anyhow, bail};
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use utils::{
    config::{endpoints::*, on_api_base_url},
    data::{ChatInfo, MessageInfo, UserInfo},
    requests::*,
};

use crate::backend::jwt::get_jwt;

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

    #[cfg(target_arch = "wasm32")]
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

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn send_decode<T>(self) -> Result<T>
    where
        T: DeserializeOwned + Clone,
    {
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
        if !response.status().is_success() {
            bail!("Request failed with status: {}", response.status())
        }

        let text = response.text().await.map_err(|e| anyhow!(e.to_string()))?;
        serde_json::from_str(&text).map_err(|e| anyhow!(e.to_string()))
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
        match get_jwt().await {
            Some(token) => {
                let mut headers = self.headers.clone();
                headers.push(("Authorization".to_string(), format!("Bearer {}", token)));
                return Self {
                    url: self.url,
                    method: self.method,
                    body: self.body,
                    headers,
                };
            }
            None => {
                error!("Failed to get JWT for request");
            }
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

pub async fn list_chats() -> Result<Vec<ChatInfo>> {
    let response = Request::get(&on_api_base_url(chats::IG_LIST).await)
        .add_jwt()
        .await
        .build()
        .send_decode::<ListChatsResponse>()
        .await?;
    Ok(response.0)
}

pub async fn get_chat(uuid: Uuid) -> Result<ChatInfo> {
    let request = GetChatRequest(uuid);
    let response = Request::post(&on_api_base_url(chats::IP_GET).await)
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
    let response = Request::post(&on_api_base_url(chats::IP_VERIFY_PRIVATE).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<NewChatResponse>()
        .await?;
    Ok(response.0)
}

pub async fn new_group(title: String, members: Vec<Uuid>) -> Result<Uuid> {
    let request = NewGroupRequest { title, members };
    let response = Request::post(&on_api_base_url(groups::IP_NEW).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<NewChatResponse>()
        .await?;
    Ok(response.0)
}

#[cfg(target_arch = "wasm32")]
pub async fn natives_authenticate(key: Uuid) -> Result<()> {
    let request = NativesAuthenticateRequest(key);
    let response = Request::post(&on_api_base_url(natives::IP_AUTHENTICATE).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<NativesAuthenticateResponse>()
        .await?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn natives_is_authenticated(key: Uuid) -> Result<Option<String>> {
    let request = NativesIsAuthenticatedRequest(key);
    let response = Request::post(&on_api_base_url(natives::IP_IS_AUTHENTICATED).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<NativesIsAuthenticatedResponse>()
        .await?;
    Ok(response.0)
}

pub async fn list_messages(chat_uuid: Uuid) -> Result<Vec<MessageInfo>> {
    let request = ListMessagesRequest(chat_uuid);
    let response = Request::post(&on_api_base_url(messages::IP_LIST).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<ListMessagesResponse>()
        .await?;
    Ok(response.0)
}

pub async fn send_message(chat_uuid: Uuid, content: String, reply: Option<Uuid>) -> Result<()> {
    let request = SendMessageRequest {
        chat_uuid,
        content,
        reply,
    };
    let _ = Request::post(&on_api_base_url(messages::IP_SEND).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<SendMessageResponse>()
        .await?;
    Ok(())
}

pub async fn delete_message(uuid: Uuid) -> Result<()> {
    let request = DeleteMessageRequest(uuid);
    Request::post(&on_api_base_url(messages::IP_DELETE).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<DeleteMessageResponse>()
        .await?;
    Ok(())
}

pub async fn edit_message(uuid: Uuid, new_content: String) -> Result<()> {
    let request = EditMessageRequest { uuid, new_content };
    Request::post(&on_api_base_url(messages::IP_EDIT).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<DeleteMessageResponse>()
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn check_user() -> Result<bool> {
    let response = Request::get(&on_api_base_url(users::IG_CHECK).await)
        .build()
        .send_decode::<CheckUserResponse>()
        .await?;
    Ok(response.0)
}

pub async fn my_user() -> Result<UserInfo> {
    let response = Request::get(&on_api_base_url(users::IG_ME).await)
        .add_jwt()
        .await
        .build()
        .send_decode::<GetUserResponse>()
        .await?;
    Ok(response.0)
}

pub async fn get_username(username: String) -> Result<UserInfo> {
    let request = GetUsernameRequest(username);
    let response = Request::post(&on_api_base_url(users::IP_NAME).await)
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
    let _ = Request::post(&on_api_base_url(users::IP_SETUP).await)
        .add_body_from_json(&request)
        .build()
        .send_decode::<SetupUserResponse>()
        .await?;
    Ok(())
}

pub async fn list_users(exclude_self: bool) -> Result<Vec<UserInfo>> {
    let request = ListUsersRequest { exclude_self };
    let response = Request::post(&on_api_base_url(users::IP_LIST).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<ListUsersResponse>()
        .await?;
    Ok(response.0)
}

pub async fn chat_users(chat_uuid: Uuid) -> Result<Vec<UserInfo>> {
    let request = ChatUsersRequest(chat_uuid);
    let response = Request::post(&on_api_base_url(users::IP_CHAT).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<ListUsersResponse>()
        .await?;
    Ok(response.0)
}
