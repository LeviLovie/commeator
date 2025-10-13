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

#[post("/api/chats/get")]
pub async fn get_chat(jwt: String, chat_id: i32) -> Result<ChatInfo, ServerFnError> {
    let user = verify_jwt(&jwt).await?;
    let db = db().await;

    let is_member = ChatMembers::find()
        .filter(chat_members::Column::ChatId.eq(chat_id))
        .filter(chat_members::Column::UserId.eq(user.id))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .is_some();

    if !is_member {
        return Err(ServerFnError::new("Not a member of the chat".to_string()));
    }

    let chat_model: chats::Model = Chats::find_by_id(chat_id)
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Chat not found".to_string()))?;

    let chat = ChatInfo {
        id: chat_model.id,
        name: chat_model.name,
        is_group: chat_model.is_group,
    };

    Ok(chat)
}

#[post("/api/chats/private/verify")]
pub async fn verify_private_chat(jwt: String, with_user: String) -> Result<i32, ServerFnError> {
    let user = verify_jwt(&jwt).await?;
    let db = db().await;

    let other_user: users::Model = Users::find()
        .filter(users::Column::Username.eq(with_user))
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("User not found".to_string()))?;
    if other_user.id == user.id {
        return Err(ServerFnError::new("Cannot create a chat with yourself".to_string()));
    }

    let existing_chat = chats::Entity::find()
        .join(JoinType::InnerJoin, chats::Relation::ChatMembers.def())
        .filter(chats::Column::IsGroup.eq(false))
        .filter(chat_members::Column::UserId.eq(user.id))
        .filter(
            chat_members::Column::ChatId.in_subquery(
                Query::select()
                    .column(chat_members::Column::ChatId)
                    .from(chat_members::Entity)
                    .and_where(chat_members::Column::UserId.eq(other_user.id))
                    .to_owned()
            )
        )
        .one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    if existing_chat.is_some() {
        return Ok(existing_chat.unwrap().id);
    }

    let new_chat_model = chats::ActiveModel {
        name: Set(format!("{} & {}", user.nickname, other_user.nickname)),
        is_group: Set(false),
        ..Default::default()
    };
    let new_chat = new_chat_model
        .insert(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let first_chat_member_model = chat_members::ActiveModel {
        chat_id: Set(new_chat.id),
        user_id: Set(user.id),
        joined_at: Set(chrono::Utc::now().naive_utc()),
    };
    first_chat_member_model
        .insert(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let second_chat_member_model = chat_members::ActiveModel {
        chat_id: Set(new_chat.id),
        user_id: Set(other_user.id),
        joined_at: Set(chrono::Utc::now().naive_utc()),
    };
    second_chat_member_model
        .insert(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(new_chat.id)
}
