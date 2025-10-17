use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenerateJwtResponse {
    pub jwt: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyJwtResponse(pub bool);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChatInfo {
    pub uuid: Uuid,
    pub name: String,
    pub is_group: bool,
}

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
pub struct VerifyPrivateChatResponse(pub Uuid);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MessageInfo {
    pub uuid: Uuid,
    pub sender_uuid: Uuid,
    pub sender_nickname: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub edited_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListMessagesRequest(pub Uuid);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListMessagesResponse(pub Vec<MessageInfo>);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMessageRequest {
    pub chat_uuid: Uuid,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMessageResponse {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub uuid: Uuid,
    pub email: String,
    pub username: String,
    pub nickname: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckUserResponse(pub bool);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserRequest(pub Uuid);

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
