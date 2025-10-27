use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::{ChatInfo, MessageInfo, UserInfo};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenerateJwtResponse {
    pub jwt: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyJwtResponse(pub bool);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListChatsResponse(pub Vec<ChatInfo>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetChatRequest(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetChatResponse(pub ChatInfo);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyPrivateChatRequest {
    pub with_user: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewChatResponse(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewGroupRequest {
    pub title: String,
    pub members: Vec<Uuid>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListMessagesRequest(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListMessagesResponse(pub Vec<MessageInfo>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMessageRequest {
    pub chat_uuid: Uuid,
    pub content: String,
    pub reply: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMessageResponse {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteMessageRequest(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeleteMessageResponse {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EditMessageRequest {
    pub uuid: Uuid,
    pub new_content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EditMessageResponse {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckUserResponse(pub bool);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserRequest(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUsernameRequest(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserResponse(pub UserInfo);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetupUserRequest {
    pub username: String,
    pub nickname: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetupUserResponse {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUsersRequest {
    pub exclude_self: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUsersResponse(pub Vec<UserInfo>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatUsersRequest(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NativesAuthenticateRequest(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NativesAuthenticateResponse {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NativesIsAuthenticatedRequest(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NativesIsAuthenticatedResponse(pub Option<String>);
