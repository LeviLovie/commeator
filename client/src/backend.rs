use anyhow::{Result, anyhow, bail};
use chrono::{NaiveDateTime, Utc};
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use std::sync::{LazyLock, Mutex};
use uuid::Uuid;

use crate::{components::logout, Route};
use utils::{
    auth::KratosUserData,
    config::{
        endpoints::{auth::{URI_LOGIN, URI_WHOAMI}, *}, on_api_base_url, on_auth_base_url,
    },
    requests::*,
};

static JWT: LazyLock<Mutex<Option<(String, NaiveDateTime)>>> = LazyLock::new(|| Mutex::new(None));
pub static CENTRIFUGO_JWT: LazyLock<Mutex<Option<(String, NaiveDateTime)>>> = LazyLock::new(|| Mutex::new(None));

async fn regenerate_centrifugo_jwt() {
    match generate_centrifugo_jwt().await {
        Ok(token) => {
            *CENTRIFUGO_JWT.lock().unwrap() = Some(token);
        }
        Err(e) => {
            if e.to_string().contains("User not found") {
                logout().await;
            }
            error!("Failed to request centrifugo JWT: {}", e);
        }
    }
}

pub async fn get_centrifugo_jwt() -> Option<String> {
    {
        let jwt = {
            let guard = CENTRIFUGO_JWT.lock().unwrap();
            guard.clone()
        };
        match jwt {
            None => regenerate_centrifugo_jwt().await,
            Some((_, expires_at)) if expires_at <= Utc::now().naive_utc() => {
                regenerate_centrifugo_jwt().await
            }
            _ => {}
        }
    };

    if let Some(token) = CENTRIFUGO_JWT.lock().unwrap().as_ref() {
        Some(token.0.clone())
    } else {
        warn!("Centrifugo JWT is still None after regeneration attempt");
        None
    }
}

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

    async fn regenerate_jwt() {
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

    pub async fn add_jwt(self) -> Self {
        {
            let jwt = {
                let guard = JWT.lock().unwrap();
                guard.clone()
            };
            match jwt {
                None => Self::regenerate_jwt().await,
                Some((_, expires_at)) if expires_at <= Utc::now().naive_utc() => {
                    Self::regenerate_jwt().await
                }
                _ => {}
            }
        };

        if let Some(token) = JWT.lock().unwrap().as_ref() {
            let mut headers = self.headers.clone();
            headers.push(("Authorization".to_string(), format!("Bearer {}", token.0)));
            return Self {
                url: self.url,
                method: self.method,
                body: self.body,
                headers,
            };
        } else {
            warn!("JWT is still None after regeneration attempt");
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

pub async fn generate_jwt() -> Result<(String, NaiveDateTime)> {
    let response = Request::get(&on_api_base_url(jwt::IG_GENERATE).await)
        .build()
        .send_decode::<GenerateJwtResponse>()
        .await?;
    Ok((response.jwt, response.expires_at))
}

pub async fn generate_centrifugo_jwt() -> Result<(String, NaiveDateTime)> {
    let response = Request::get(&on_api_base_url(jwt::IG_GENERATE_CENTRIFUGO).await)
        .add_jwt()
        .await
        .build()
        .send_decode::<GenerateJwtResponse>()
        .await?;
    Ok((response.jwt, response.expires_at))
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
        .send_decode::<VerifyPrivateChatResponse>()
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

pub async fn send_message(chat_uuid: Uuid, content: String) -> Result<()> {
    let request = SendMessageRequest { chat_uuid, content };
    let _ = Request::post(&on_api_base_url(messages::IP_SEND).await)
        .add_body_from_json(&request)
        .add_jwt()
        .await
        .build()
        .send_decode::<SendMessageResponse>()
        .await?;
    Ok(())
}

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

pub async fn get_user(uuid: Uuid) -> Result<UserInfo> {
    let request = GetUserRequest(uuid);
    let response = Request::post(&on_api_base_url(users::IP_GET).await)
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

async fn try_get_kratos_user() -> Result<KratosUserData> {
    Request::get(&on_auth_base_url(URI_WHOAMI).await)
        .build()
        .send_decode::<KratosUserData>()
        .await
}

pub async fn get_kratos_user() -> Option<KratosUserData> {
    match try_get_kratos_user().await {
        Ok(user) => Some(user),
        Err(e) => {
            info!("Error getting Kratos user: {}", e);
            navigator().replace(on_auth_base_url(URI_LOGIN).await);
            None
        }
    }
}
