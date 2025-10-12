use dioxus::prelude::*;
pub use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
mod server_utils {
    pub use crate::backend::server_utils::*;
}
#[cfg(feature = "server")]
use server_utils::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ChatInfo {
    pub id: i32,
    pub name: String,
    pub is_group: bool,
}

#[post("/api/chats/list")]
pub async fn list_chats(jwt: String) -> Result<Vec<ChatInfo>, ServerFnError> {
    let user = verify_jwt(&jwt).await?;
    let db = db().await;

    let chat_ids: Vec<chat_members::Model> = ChatMembers::find()
        .filter(chat_members::Column::UserId.eq(user.id))
        .all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let chat_models: Vec<chats::Model> = Chats::find()
        .filter(chats::Column::Id.is_in(chat_ids.iter().map(|cm| cm.chat_id)))
        .all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let chats = chat_models
        .into_iter()
        .map(|chat| ChatInfo {
            id: chat.id,
            name: chat.name,
            is_group: chat.is_group,
        })
        .collect();

    Ok(chats)
}
