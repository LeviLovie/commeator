use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data::{ChatInfo, MessageInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
pub enum Update {
    NewMessage(MessageInfo),
    DeleteMessage(DeleteMessagePayload),
    UpdateMessage(UpdateMessagePayload),
    NewChat(ChatInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMessagePayload {
    pub chat_uuid: Uuid,
    pub message_uuid: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMessagePayload {
    pub uuid: Uuid,
    pub new_content: String,
    pub edited_at: NaiveDateTime,
}
