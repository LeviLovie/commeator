use serde::{Deserialize, Serialize};

use crate::data::{ChatInfo, MessageInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
pub enum Update {
    NewMessage(MessageInfo),
    NewChat(ChatInfo),
}
