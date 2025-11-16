use chrono::DateTime;
use sea_orm::QueryOrder;

use super::prelude::*;

const MAX_MESSAGE_LEN: usize = 1024 * 16; // 16 KB
const PAGE_SIZE: u64 = 32;

pub fn routes() -> Vec<rocket::Route> {
    routes![send, edit, delete, list, sync]
}

#[post("/send", data = "<req>")]
pub async fn send(
    jwt: Jwt,
    req: Proto<SendMessage>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<SendMessageResp>> {
    let chat_uuid = Uuid::parse_str(&req.0.chat_uuid).context("Invalid chat UUID format")?;

    let content = req.0.content.trim();

    if content.len() > MAX_MESSAGE_LEN {
        return Err(anyhow!("Message too long").into());
    }

    if content.is_empty() {
        return Err(anyhow!("Message content cannot be empty").into());
    }

    chat_members::Entity::find()
        .filter(chat_members::Column::ChatUuid.eq(chat_uuid))
        .filter(chat_members::Column::UserUuid.eq(jwt.0.sub))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or_else(|| anyhow::anyhow!("Access denied"))?;

    let reply_to = if let Some(reply_to) = req.0.reply_to {
        let reply_to_uuid =
            Uuid::parse_str(&reply_to).context("Invalid UUID format for 'reply_to'")?;

        let message_model = messages::Entity::find_by_id(reply_to_uuid)
            .one(&db.0)
            .await
            .context("Database query failed")?
            .ok_or_else(|| anyhow::anyhow!("Reply target message not found"))?;

        if message_model.chat_uuid != chat_uuid {
            return Err(anyhow!("Reply not found").into());
        }

        Some(reply_to_uuid)
    } else {
        None
    };

    messages::ActiveModel {
        chat_uuid: Set(chat_uuid),
        sender_uuid: Set(jwt.0.sub),
        content: Set(content.to_string()),
        reply: Set(reply_to),
        ..Default::default()
    }
    .insert(&db.0)
    .await
    .context("Failed to insert message into database")?;

    Ok(ProtoResp(SendMessageResp {}))
}

#[post("/edit", data = "<req>")]
pub async fn edit(
    jwt: Jwt,
    req: Proto<EditMessage>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<EditMessageResp>> {
    let message_uuid = Uuid::parse_str(&req.0.uuid).context("Invalid message UUID format")?;

    let new_content = req.0.new_content.clone();

    if new_content.is_empty() {
        return Err(anyhow!("Message content cannot be empty").into());
    }

    if new_content.len() > MAX_MESSAGE_LEN {
        return Err(anyhow!("Message too long").into());
    }

    let message_model = messages::Entity::find()
        .filter(messages::Column::Uuid.eq(message_uuid))
        .filter(messages::Column::SenderUuid.eq(jwt.0.sub))
        .filter(messages::Column::Deleted.eq(false))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or_else(|| anyhow::anyhow!("Message not found"))?;

    chat_members::Entity::find()
        .filter(chat_members::Column::ChatUuid.eq(message_model.chat_uuid))
        .filter(chat_members::Column::UserUuid.eq(jwt.0.sub))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or_else(|| anyhow::anyhow!("Access denied"))?;

    let mut message_active_model = message_model.into_active_model();
    message_active_model.content = Set(new_content.clone());
    message_active_model.edited_at = Set(Some(Utc::now().naive_utc()));
    message_active_model.updated_at = Set(Utc::now().naive_utc());
    message_active_model
        .update(&db.0)
        .await
        .context("Failed to update message in database")?;

    Ok(ProtoResp(EditMessageResp {}))
}

#[post("/delete", data = "<req>")]
pub async fn delete(
    jwt: Jwt,
    req: Proto<DeleteMessage>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<DeleteMessageResp>> {
    let message_uuid = Uuid::parse_str(&req.0.uuid).context("Invalid message UUID format")?;

    let message_model = messages::Entity::find()
        .filter(messages::Column::Uuid.eq(message_uuid))
        .filter(messages::Column::SenderUuid.eq(jwt.0.sub))
        .filter(messages::Column::Deleted.eq(false))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or_else(|| anyhow::anyhow!("Message not found"))?;

    chat_members::Entity::find()
        .filter(chat_members::Column::ChatUuid.eq(message_model.chat_uuid))
        .filter(chat_members::Column::UserUuid.eq(jwt.0.sub))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or_else(|| anyhow::anyhow!("Access denied"))?;

    let mut message_active_model = message_model.into_active_model();
    message_active_model.content = Set("[deleted]".to_string());
    message_active_model.deleted = Set(true);
    message_active_model.updated_at = Set(Utc::now().naive_utc());
    message_active_model
        .update(&db.0)
        .await
        .context("Failed to update message in database")?;

    Ok(ProtoResp(DeleteMessageResp {}))
}

#[post("/list", data = "<req>")]
pub async fn list(
    jwt: Jwt,
    req: Proto<ListMessages>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<ListMessagesResp>> {
    let chat_uuid = Uuid::parse_str(&req.0.chat_uuid).context("Invalid chat UUID format")?;

    chat_members::Entity::find()
        .filter(chat_members::Column::ChatUuid.eq(chat_uuid))
        .filter(chat_members::Column::UserUuid.eq(jwt.0.sub))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or_else(|| anyhow!("Access denied"))?;

    let ts = DateTime::from_timestamp_millis(req.0.timestamp)
        .ok_or_else(|| anyhow!("Invalid timestamp"))?;

    let mut query = messages::Entity::find()
        .filter(messages::Column::Deleted.eq(false))
        .filter(messages::Column::ChatUuid.eq(chat_uuid))
        .filter(messages::Column::UpdatedAt.lte(ts));

    query = if req.0.reversed {
        query.order_by_asc(messages::Column::UpdatedAt)
    } else {
        query.order_by_desc(messages::Column::UpdatedAt)
    };

    let rows = query
        .limit(PAGE_SIZE)
        .all(&db.0)
        .await
        .context("Database query failed")?;

    let mut updates = Vec::with_capacity(rows.len());

    for row in &rows {
        let reply_to = if let Some(reply_uuid) = row.reply {
            let reply_message_model = messages::Entity::find()
                .filter(messages::Column::Uuid.eq(reply_uuid))
                .filter(messages::Column::ChatUuid.eq(chat_uuid))
                .one(&db.0)
                .await
                .context("Database query failed")?
                .ok_or_else(|| anyhow::anyhow!("Reply target message not found"))?;

            if reply_message_model.deleted {
                None
            } else {
                Some(MessageNoReply {
                    uuid: reply_message_model.uuid.to_string(),
                    sender_uuid: reply_message_model.sender_uuid.to_string(),
                    content: reply_message_model.content.clone(),
                    created_at: reply_message_model.created_at.and_utc().timestamp_millis(),
                    edited_at: reply_message_model
                        .edited_at
                        .map(|t| t.and_utc().timestamp_millis())
                })
            }
        } else {
            None
        };

        updates.push(Message {
            uuid: row.uuid.to_string(),
            sender_uuid: row.sender_uuid.to_string(),
            content: row.content.clone(),
            reply_to,
            created_at: row.created_at.and_utc().timestamp_millis(),
            edited_at: row
                .edited_at
                .map(|t| t.and_utc().timestamp_millis()),
        });
    }

    let newest_update = rows
        .last()
        .map(|m| m.updated_at.and_utc().timestamp_millis())
        .unwrap_or(req.0.timestamp);

    Ok(ProtoResp(ListMessagesResp {
        messages: updates,
        newest_update,
    }))
}

#[post("/sync", data = "<req>")]
pub async fn sync(
    jwt: Jwt,
    req: Proto<SyncMessages>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<SyncMessagesResp>> {
    let chat_uuid = Uuid::parse_str(&req.0.chat_uuid).context("Invalid UUID format")?;

    chat_members::Entity::find()
        .filter(chat_members::Column::ChatUuid.eq(chat_uuid))
        .filter(chat_members::Column::UserUuid.eq(jwt.0.sub))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or(anyhow!("Access denied"))?;

    let since =
        DateTime::from_timestamp_millis(req.0.last_update).ok_or(anyhow!("Invalid timestamp"))?;

    println!(
        "Syncing messages for chat {} since {}",
        chat_uuid, since
    );

    let rows = messages::Entity::find()
        .filter(messages::Column::ChatUuid.eq(chat_uuid))
        .filter(messages::Column::UpdatedAt.gt(since))
        .order_by_asc(messages::Column::UpdatedAt)
        .limit(PAGE_SIZE)
        .all(&db.0)
        .await
        .context("Database query failed")?;

    println!("Syncing {} messages for chat {}", rows.len(), chat_uuid);

    let mut res_updates = vec![];

    for row in &rows {
        let update_type = if row.deleted {
            MessageUpdateType::Deleted
        } else if row.edited_at.unwrap_or(row.created_at) > row.created_at {
            MessageUpdateType::Edited
        } else {
            MessageUpdateType::New
        };

        let reply_to = if let Some(reply_uuid) = row.reply {
            match messages::Entity::find()
                .filter(messages::Column::Uuid.eq(reply_uuid))
                .filter(messages::Column::ChatUuid.eq(chat_uuid))
                .one(&db.0)
                .await
                .unwrap_or(None)
            {
                Some(m) => {
                    if m.deleted {
                        None
                    } else {
                        Some(MessageNoReply {
                            uuid: m.uuid.to_string(),
                            sender_uuid: m.sender_uuid.to_string(),
                            content: m.content.clone(),
                            created_at: m.created_at.and_utc().timestamp_millis(),
                            edited_at: m
                                .edited_at
                                .map(|t| t.and_utc().timestamp_millis())
                        })
                    }
                },
                None => None,
            }
        } else {
            None
        };

        println!(
            "Message {} update type: {:?}",
            row.uuid,
            update_type
        );

        res_updates.push(MessageUpdate {
            uuid: row.uuid.to_string(),
            updated_at: row.updated_at.and_utc().timestamp_millis(),
            r#type: update_type.into(),
            message: if update_type == MessageUpdateType::Deleted {
                None
            } else {
                Some(Message {
                    uuid: row.uuid.to_string(),
                    sender_uuid: row.sender_uuid.to_string(),
                    content: row.content.clone(),
                    reply_to,
                    created_at: row.created_at.and_utc().timestamp_millis(),
                    edited_at: row
                        .edited_at
                        .map(|t| t.and_utc().timestamp_millis())
                })
            },
        });
    }

    println!(
        "Returning {} updates for chat {}",
        res_updates.len(),
        chat_uuid
    );

    Ok(ProtoResp(SyncMessagesResp {
        newest_update: rows
            .last()
            .map(|m| m.updated_at.and_utc().timestamp_millis())
            .unwrap_or(req.0.last_update),
        has_more: rows.len() as u64 == PAGE_SIZE,
        updates: res_updates,
    }))
}
