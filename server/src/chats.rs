use axum::{
    http::HeaderMap, response::{IntoResponse, Response}, Json, 
};
use sea_orm::{sea_query::Query, ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait};
use anyhow::{anyhow, Context};

use crate::{db, schema::*, verify_jwt, AppError};
use utils::requests::{ChatInfo, GetChatRequest, GetChatResponse, ListChatsResponse, VerifyPrivateChatRequest, VerifyPrivateChatResponse};

pub async fn list_chats(headers: HeaderMap) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;
    let db = db().await;

    let chat_ids: Vec<chat_members::Model> = ChatMembers::find()
        .filter(chat_members::Column::UserUuid.eq(user.uuid))
        .all(db)
        .await
        .context("Failed to query chat memberships from database")?;

    let chat_models: Vec<chats::Model> = Chats::find()
        .filter(chats::Column::Uuid.is_in(chat_ids.iter().map(|cm| cm.chat_uuid)))
        .all(db)
        .await
        .context("Failed to query chats from database")?;

    let chats = chat_models
        .into_iter()
        .map(|chat| ChatInfo {
            uuid: chat.uuid,
            name: chat.name,
            is_group: chat.is_group,
        })
        .collect();

    let response = ListChatsResponse(chats);
    Ok(Json(response).into_response())
}

pub async fn get_chat(headers: HeaderMap, Json(body): Json<GetChatRequest>) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;
    let db = db().await;

    let chat_uuid = body.0;

    let is_member = ChatMembers::find()
        .filter(chat_members::Column::ChatUuid.eq(chat_uuid))
        .filter(chat_members::Column::UserUuid.eq(user.uuid))
        .one(db)
        .await
        .context("Failed to query chat membership from database")?
        .is_some();

    if !is_member {
        return Err(anyhow!("User is not a member of this chat").into());
    }

    let chat_model: chats::Model = Chats::find_by_id(chat_uuid)
        .one(db)
        .await
        .context("Failed to query chat from database")?
        .ok_or_else(|| anyhow!("Chat not found"))?;

    let chat = ChatInfo {
        uuid: chat_model.uuid,
        name: chat_model.name,
        is_group: chat_model.is_group,
    };

    let response = GetChatResponse(chat);
    Ok(Json(response).into_response())
}

pub async fn verify_private_chat(headers: HeaderMap, Json(body): Json<VerifyPrivateChatRequest>) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;
    let db = db().await;

    let other_user: users::Model = Users::find()
        .filter(users::Column::Username.eq(body.with_user))
        .one(db)
        .await
        .context("Failed to query user from database")?
        .ok_or_else(|| anyhow!("User not found"))?;

    if other_user.uuid == user.uuid {
        return Err(anyhow!("Cannot create a private chat with yourself").into());
    }

    let existing_chat = chats::Entity::find()
        .join(JoinType::InnerJoin, chats::Relation::ChatMembers.def())
        .filter(chats::Column::IsGroup.eq(false))
        .filter(chat_members::Column::UserUuid.eq(user.uuid))
        .filter(
            chat_members::Column::ChatUuid.in_subquery(
                Query::select()
                    .column(chat_members::Column::ChatUuid)
                    .from(chat_members::Entity)
                    .and_where(chat_members::Column::UserUuid.eq(other_user.uuid))
                    .to_owned()
            )
        )
        .one(db)
        .await
        .context("Failed to query existing private chat from database")?;

    if let Some(chat) = existing_chat {
        let response = VerifyPrivateChatResponse(chat.uuid);
        return Ok(Json(response).into_response());
    }

    let new_chat_model = chats::ActiveModel {
        name: Set(format!("{} & {}", user.nickname, other_user.nickname)),
        is_group: Set(false),
        ..Default::default()
    };
    let new_chat = new_chat_model
        .insert(db)
        .await
        .context("Failed to create new chat in database")?;

    let first_chat_member_model = chat_members::ActiveModel {
        chat_uuid: Set(new_chat.uuid),
        user_uuid: Set(user.uuid),
        joined_at: Set(chrono::Utc::now().naive_utc()),
    };
    first_chat_member_model
        .insert(db)
        .await
        .context("Failed to add first user to new chat in database")?;

    let second_chat_member_model = chat_members::ActiveModel {
        chat_uuid: Set(new_chat.uuid),
        user_uuid: Set(other_user.uuid),
        joined_at: Set(chrono::Utc::now().naive_utc()),
    };
    second_chat_member_model
        .insert(db)
        .await
        .context("Failed to add second user to new chat in database")?;

    let response = VerifyPrivateChatResponse(new_chat.uuid);
    Ok(Json(response).into_response())
}
