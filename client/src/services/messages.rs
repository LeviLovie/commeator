use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use dioxus::prelude::warn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{request::backend, services};
use proto::{ListMessages, ListMessagesResp, MessageUpdateType, SyncMessages, SyncMessagesResp};

const PERSISTENT_MESSAGES: u32 = 256;

pub struct Messages {
    storage: services::Storage,
}

impl Messages {
    pub fn new() -> Self {
        Self {
            storage: services::Storage::new(),
        }
    }

    pub async fn load(&self, jwt: &str, chat_uuid: Uuid) -> Result<Chat> {
        let file = self
            .storage
            .load_string(Self::filename(chat_uuid))
            .await
            .context("Failed to load chat from storage")?;

        if file.is_empty() || ron::from_str::<Chat>(&file).is_err() || file == "\"\"" {
            warn!("No local chat found, fetching from backend");
            let timestamp = Self::now();
            let last_messages = backend::<ListMessages, ListMessagesResp>(
                "/m/list",
                jwt.clone(),
                ListMessages {
                    chat_uuid: chat_uuid.to_string(),
                    timestamp: timestamp.and_utc().timestamp_millis(),
                    reversed: true,
                },
            );
            dioxus::prelude::info!("Fetched last messages for chat {} from backend", chat_uuid);
            self.save(Chat {
                uuid: chat_uuid,
                messages: last_messages
                    .await?
                    .messages
                    .into_iter()
                    .map(Message::from)
                    .collect(),
                last_synced: timestamp,
                needs_sync: true,
            }).await;
        }

        let file = self
            .storage
            .load_string(Self::filename(chat_uuid))
            .await
            .context("Failed to load chat from storage")?;
        let mut chat: Chat = ron::from_str(&file).context("Failed to load chat from storage")?;

        chat.sync_step(jwt).await?;
        while chat.needs_sync {
            chat.sync_step(jwt).await?;
        }

        if chat.messages.len() > PERSISTENT_MESSAGES as usize {
            chat.messages
                .sort_by(|a, b| b.created_at.cmp(&a.created_at));
            chat.messages.truncate(PERSISTENT_MESSAGES as usize);
            chat.messages
                .sort_by(|a, b| a.created_at.cmp(&b.created_at));
        }

        Self::save(&self, chat.clone()).await.context("Failed to save chat after sync")?;

        Ok(chat)
    }

    pub async fn save(&self, chat: Chat) -> Result<()> {
        let serialized = ron::to_string(&chat).context("Failed to serialize chat for storage")?;
        self.storage
            .save_string(Self::filename(chat.uuid), serialized.clone()).await
            .context("Failed to save chat to storage")?;
        Ok(())
    }

    fn now() -> NaiveDateTime {
        Utc::now().naive_utc()
    }

    fn filename(chat_uuid: Uuid) -> String {
        format!("chat:{}", chat_uuid.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub uuid: Uuid,
    pub sender_uuid: String,
    pub content: String,
    pub reply_to: Option<MessageNoReply>,
    pub created_at: NaiveDateTime,
    pub edited_at: Option<NaiveDateTime>,
}

impl From<proto::Message> for Message {
    fn from(proto_msg: proto::Message) -> Self {
        Self {
            uuid: Uuid::parse_str(&proto_msg.uuid).unwrap(),
            sender_uuid: proto_msg.sender_uuid,
            content: proto_msg.content,
            reply_to: if proto_msg.reply_to.is_some() {
                Some(MessageNoReply::from(proto_msg.reply_to.unwrap()))
            } else {
                None
            },
            created_at: DateTime::from_timestamp_millis(proto_msg.created_at)
                .unwrap_or_else(|| DateTime::from_timestamp_millis(0).unwrap())
                .naive_utc(),
            edited_at: if proto_msg.edited_at.is_some() {
                let ts = proto_msg.edited_at.unwrap();
                Some(
                    DateTime::from_timestamp_millis(ts)
                        .unwrap_or_else(|| DateTime::from_timestamp_millis(0).unwrap())
                        .naive_utc(),
                )
            } else {
                None
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageNoReply {
    pub uuid: Uuid,
    pub sender_uuid: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub edited_at: Option<NaiveDateTime>,
}

impl From<proto::MessageNoReply> for MessageNoReply {
    fn from(proto_msg: proto::MessageNoReply) -> Self {
        Self {
            uuid: Uuid::parse_str(&proto_msg.uuid).unwrap(),
            sender_uuid: proto_msg.sender_uuid,
            content: proto_msg.content,
            created_at: DateTime::from_timestamp_millis(proto_msg.created_at)
                .unwrap_or_else(|| DateTime::from_timestamp_millis(0).unwrap())
                .naive_utc(),
            edited_at: if proto_msg.edited_at.is_some() {
                let ts = proto_msg.edited_at.unwrap();
                Some(
                    DateTime::from_timestamp_millis(ts)
                        .unwrap_or_else(|| DateTime::from_timestamp_millis(0).unwrap())
                        .naive_utc(),
                )
            } else {
                None
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub uuid: Uuid,
    pub messages: Vec<Message>,
    pub last_synced: NaiveDateTime,
    pub needs_sync: bool,
}

impl Chat {
    pub async fn sync_step(&mut self, jwt: &str) -> Result<()> {
        let updates = backend::<SyncMessages, SyncMessagesResp>(
            "/m/sync",
            jwt,
            SyncMessages {
                chat_uuid: self.uuid.to_string(),
                last_update: self.last_synced.and_utc().timestamp_millis(),
            },
        )
        .await
        .context("Failed to sync messages from backend")?;

        self.last_synced = DateTime::from_timestamp_millis(updates.newest_update)
            .unwrap_or_else(|| DateTime::from_timestamp_millis(0).unwrap())
            .naive_utc();
        self.needs_sync = updates.has_more;

        for update in updates.updates {
            match update.r#type.try_into().unwrap() {
                MessageUpdateType::New => {
                    if let Some(msg) = update.message && !self.messages.iter().any(|m| m.uuid.to_string() == msg.uuid) {
                        self.messages.push(Message::from(msg));
                    }
                }
                MessageUpdateType::Edited => {
                    if let Some(msg) = update.message {
                        if let Some(existing_msg) = self
                            .messages
                            .iter_mut()
                            .find(|m| m.uuid.to_string() == msg.uuid)
                        {
                            existing_msg.content = msg.content;
                            existing_msg.edited_at = if msg.edited_at.is_some() {
                                let ts = msg.edited_at.unwrap();
                                Some(
                                    DateTime::from_timestamp_millis(ts)
                                        .unwrap_or_else(|| {
                                            DateTime::from_timestamp_millis(0).unwrap()
                                        })
                                        .naive_utc(),
                                )
                            } else {
                                None
                            };
                        }
                    }
                }
                MessageUpdateType::Deleted => {
                    let update_uuid = Uuid::parse_str(&update.uuid)
                        .context("Failed to parse UUID from deleted message update")?;
                    self.messages.retain(|m| m.uuid != update_uuid);
                    for message in &mut self.messages {
                        if let Some(reply_to) = &message.reply_to {
                            if reply_to.uuid == update_uuid {
                                message.reply_to = None;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
