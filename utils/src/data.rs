use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChatInfo {
    pub uuid: Uuid,
    pub name: String,
    pub is_group: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub uuid: Uuid,
    pub email_hash: String,
    pub username: String,
    pub nickname: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MessageInfo {
    pub uuid: Uuid,
    pub sender_uuid: Uuid,
    pub content: String,
    pub reply: Option<MessageInfoNoReply>,
    pub created_at: NaiveDateTime,
    pub edited_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MessageInfoNoReply {
    pub uuid: Uuid,
    pub sender_uuid: Uuid,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub edited_at: Option<NaiveDateTime>,
}
