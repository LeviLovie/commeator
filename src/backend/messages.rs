use dioxus::prelude::*;
pub use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
mod server_utils {
    pub use crate::backend::server_utils::*;
}
#[cfg(feature = "server")]
use server_utils::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub content: String,
    pub created_at: String,
    pub edited_at: String,
}

#[post("/api/messages/list")]
pub async fn list_messages(jwt: String, chat_id: i32) -> Result<Vec<Message>, ServerFnError> {
    let user = verify_jwt(&jwt).await?;
    let db = db().await;

    let chat_member: Option<chat_members::Model> = ChatMembers::find()
        .filter(chat_members::Column::ChatId.eq(chat_id))
        .filter(chat_members::Column::UserId.eq(user.id))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    if chat_member.is_none() {
        return Err(ServerFnError::new("User is not a member of this chat".to_string()));
    }

    let message_models: Vec<messages::Model> = Messages::find()
        .filter(messages::Column::ChatId.eq(chat_id))
        .filter(messages::Column::Deleted.eq(false))
        .all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let messages = message_models
        .into_iter()
        .map(|msg| Message {
            id: msg.id,
            sender_id: msg.sender_id,
            content: msg.content,
            created_at: msg.created_at.to_string(),
            edited_at: msg.edited_at.to_string(),
        })
        .collect();

    Ok(messages)
}
