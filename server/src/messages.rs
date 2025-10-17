use anyhow::{anyhow, Context};
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{conn::publish, db, schema::*, verify_jwt, AppError};
use utils::requests::{
    ListMessagesRequest, ListMessagesResponse, MessageInfo, SendMessageRequest, SendMessageResponse,
};

pub async fn list_messages(
    headers: HeaderMap,
    Json(body): Json<ListMessagesRequest>,
) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;
    let db = db().await;

    let _: chat_members::Model = ChatMembers::find()
        .filter(chat_members::Column::ChatUuid.eq(body.0))
        .filter(chat_members::Column::UserUuid.eq(user.uuid))
        .one(db)
        .await
        .context("Failed to query chat membership from database")?
        .ok_or_else(|| anyhow!("User is not a member of this chat"))?;

    let message_models: Vec<messages::Model> = Messages::find()
        .filter(messages::Column::ChatUuid.eq(body.0))
        .filter(messages::Column::Deleted.eq(false))
        .all(db)
        .await
        .context("Failed to query messages from database")?;

    let messages = message_models
        .into_iter()
        .map(|msg| MessageInfo {
            uuid: msg.uuid,
            sender_uuid: msg.sender_uuid,
            content: msg.content,
            created_at: msg.created_at,
            edited_at: msg.edited_at,
        })
        .collect();

    let response = ListMessagesResponse(messages);
    Ok(Json(response).into_response())
}

pub async fn send_message(
    headers: HeaderMap,
    Json(body): Json<SendMessageRequest>,
) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;
    let db = db().await;

    let _: chat_members::Model = ChatMembers::find()
        .filter(chat_members::Column::ChatUuid.eq(body.chat_uuid))
        .filter(chat_members::Column::UserUuid.eq(user.uuid))
        .one(db)
        .await
        .context("Failed to query chat membership from database")?
        .ok_or_else(|| anyhow!("User is not a member of this chat"))?;

    let new_message = messages::ActiveModel {
        chat_uuid: Set(body.chat_uuid),
        sender_uuid: Set(user.uuid),
        content: Set(body.content.clone()),
        deleted: Set(false),
        ..Default::default()
    };

    let inserted_message = new_message
        .insert(db)
        .await
        .context("Failed to insert new message into database")?;

    let message = MessageInfo {
        uuid: inserted_message.uuid,
        sender_uuid: user.uuid,
        content: inserted_message.content,
        created_at: inserted_message.created_at,
        edited_at: None,
    };
    let serialized_message = serde_json::to_value(&message)
        .map_err(|e| anyhow!("Failed to serialize message info: {}", e))?;

    publish(&format!("chat_{}", body.chat_uuid), serialized_message).await?;

    let response = SendMessageResponse {};
    Ok(Json(response).into_response())
}
