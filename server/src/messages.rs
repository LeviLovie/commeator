use anyhow::{anyhow, Context};
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};

use crate::{conn::publish, db, schema::*, verify_jwt, AppError};
use utils::{
    data::{MessageInfo, MessageInfoNoReply},
    requests::{
        DeleteMessageRequest, DeleteMessageResponse, EditMessageRequest, ListMessagesRequest,
        ListMessagesResponse, SendMessageRequest, SendMessageResponse,
    },
    updates::{DeleteMessagePayload, Update, UpdateMessagePayload},
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

    let mut messages = Vec::new();
    for msg in message_models {
        let message_reply = if let Some(reply_uuid) = msg.reply {
            let reply_message: messages::Model = Messages::find()
                .filter(messages::Column::Uuid.eq(reply_uuid))
                .one(db)
                .await
                .context("Failed to query reply message from database")?
                .ok_or_else(|| anyhow!("Reply message not found"))?;
            Some(MessageInfoNoReply {
                uuid: reply_message.uuid,
                sender_uuid: reply_message.sender_uuid,
                content: reply_message.content,
                created_at: reply_message.created_at,
                edited_at: reply_message.edited_at,
            })
        } else {
            None
        };

        messages.push(MessageInfo {
            uuid: msg.uuid,
            sender_uuid: msg.sender_uuid,
            content: msg.content,
            reply: message_reply,
            created_at: msg.created_at,
            edited_at: msg.edited_at,
        });
    }

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
        reply: Set(body.reply),
        deleted: Set(false),
        ..Default::default()
    };

    let inserted_message = new_message
        .insert(db)
        .await
        .context("Failed to insert new message into database")?;

    let message_reply = if let Some(reply_uuid) = body.reply {
        let reply_message: messages::Model = Messages::find()
            .filter(messages::Column::Uuid.eq(reply_uuid))
            .one(db)
            .await
            .context("Failed to query reply message from database")?
            .ok_or_else(|| anyhow!("Reply message not found"))?;
        Some(MessageInfoNoReply {
            uuid: reply_message.uuid,
            sender_uuid: reply_message.sender_uuid,
            content: reply_message.content,
            created_at: reply_message.created_at,
            edited_at: reply_message.edited_at,
        })
    } else {
        None
    };

    let message = MessageInfo {
        uuid: inserted_message.uuid,
        sender_uuid: user.uuid,
        content: inserted_message.content,
        reply: message_reply,
        created_at: inserted_message.created_at,
        edited_at: None,
    };
    let update = Update::NewMessage(message);
    publish(&format!("chat_{}", body.chat_uuid), update).await?;

    let response = SendMessageResponse {};
    Ok(Json(response).into_response())
}

pub async fn delete_message(
    headers: HeaderMap,
    Json(body): Json<DeleteMessageRequest>,
) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;
    let db = db().await;

    let message: messages::Model = Messages::find()
        .filter(messages::Column::Uuid.eq(body.0))
        .one(db)
        .await
        .context("Failed to query message from database")?
        .ok_or_else(|| anyhow!("User is not a member of this chat"))?;

    if message.sender_uuid != user.uuid {
        return Err(anyhow!("User is not the sender of this message").into());
    }

    let mut message_active: messages::ActiveModel = message.into();
    message_active.deleted = Set(true);
    let message = message_active
        .update(db)
        .await
        .context("Failed to delete message in database")?;

    let update = Update::DeleteMessage(DeleteMessagePayload {
        chat_uuid: message.chat_uuid,
        message_uuid: message.uuid,
    });
    publish(&format!("chat_{}", message.chat_uuid), update).await?;

    let response = DeleteMessageResponse {};
    Ok(Json(response).into_response())
}

pub async fn edit_message(
    headers: HeaderMap,
    Json(body): Json<EditMessageRequest>,
) -> Result<Response, AppError> {
    let user = verify_jwt(&headers).await?;
    let db = db().await;

    let message: messages::Model = Messages::find()
        .filter(messages::Column::Uuid.eq(body.uuid))
        .one(db)
        .await
        .context("Failed to query message from database")?
        .ok_or_else(|| anyhow!("User is not a member of this chat"))?;

    if message.sender_uuid != user.uuid {
        return Err(anyhow!("User is not the sender of this message").into());
    }

    let mut message_active: messages::ActiveModel = message.into();
    message_active.content = Set(body.new_content.clone());
    message_active.edited_at = Set(Some(chrono::Utc::now().naive_utc()));
    let message = message_active
        .update(db)
        .await
        .context("Failed to delete message in database")?;

    let update = Update::UpdateMessage(UpdateMessagePayload {
        uuid: message.uuid,
        new_content: message.content,
        edited_at: message.edited_at.unwrap(),
    });
    publish(&format!("chat_{}", message.chat_uuid), update).await?;

    let response = DeleteMessageResponse {};
    Ok(Json(response).into_response())
}
