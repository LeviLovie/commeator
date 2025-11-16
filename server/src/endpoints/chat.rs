use super::prelude::*;

const MAX_CHAT_NAME_LENGTH: usize = 100;

pub fn routes() -> Vec<rocket::Route> {
    routes![list, get, verify, g_create]
}

#[post("/list")]
pub async fn list(jwt: Jwt, db: &State<Db>) -> ApiResult<ProtoResp<ListChatsResp>> {
    let cm = Alias::new("cm");
    let cn = Alias::new("cn");

    let chat_rows = chats::Entity::find()
        .join_as(
            JoinType::InnerJoin,
            RelationDef::from(
                chat_members::Entity::belongs_to(chats::Entity)
                    .from(chat_members::Column::ChatUuid)
                    .to(chats::Column::Uuid),
            )
            .rev(),
            cm.clone(),
        )
        .join_as(
            JoinType::LeftJoin,
            RelationDef::from(
                chat_names::Entity::belongs_to(chats::Entity)
                    .from(chat_names::Column::ChatUuid)
                    .to(chats::Column::Uuid),
            )
            .rev(),
            cn.clone(),
        )
        .filter(Expr::col((cm.clone(), chat_members::Column::UserUuid)).eq(jwt.0.sub))
        .filter(Expr::col((cn.clone(), chat_names::Column::UserUuid)).eq(jwt.0.sub))
        .select_only()
        .column(chats::Column::Uuid)
        .column(chats::Column::IsGroup)
        .expr_as(
            Expr::col((cn.clone(), chat_names::Column::Name)),
            "chat_name",
        )
        .into_tuple::<(Uuid, bool, Option<String>)>()
        .all(&db.0)
        .await
        .context("Database query failed")?;

    let chats = chat_rows
        .into_iter()
        .map(|(uuid, is_group, maybe_name)| Chat {
            uuid: uuid.to_string(),
            is_group,
            name: maybe_name.unwrap_or_else(|| "Unnamed".into()),
        })
        .collect();

    Ok(ProtoResp(ListChatsResp { chats }))
}

#[post("/get", data = "<req>")]
pub async fn get(
    jwt: Jwt,
    req: Proto<GetChat>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<GetChatResp>> {
    let chat_uuid = Uuid::parse_str(&req.0.uuid)
        .context("Invalid chat UUID format")?;
    
    let cm = Alias::new("cm");
    let cn = Alias::new("cn");

    let row = chats::Entity::find()
        .join_as(
            JoinType::InnerJoin,
            RelationDef::from(
                chat_members::Entity::belongs_to(chats::Entity)
                    .from(chat_members::Column::ChatUuid)
                    .to(chats::Column::Uuid),
            )
            .rev(),
            cm.clone(),
        )
        .join_as(
            JoinType::LeftJoin,
            RelationDef::from(
                chat_names::Entity::belongs_to(chats::Entity)
                    .from(chat_names::Column::ChatUuid)
                    .to(chats::Column::Uuid),
            )
            .rev(),
            cn.clone(),
        )
        .filter(Expr::col((cm.clone(), chat_members::Column::UserUuid)).eq(jwt.0.sub))
        .filter(Expr::col((cn.clone(), chat_names::Column::UserUuid)).eq(jwt.0.sub))
        .filter(chats::Column::Uuid.eq(chat_uuid))
        .select_only()
        .column(chats::Column::Uuid)
        .column(chats::Column::IsGroup)
        .expr_as(
            Expr::col((cn.clone(), chat_names::Column::Name)),
            "chat_name",
        )
        .into_tuple::<(Uuid, bool, Option<String>)>()
        .one(&db.0)
        .await
        .context("Database query failed")?;

    let Some((uuid, is_group, maybe_name)) = row else {
        return Err(anyhow!("Chat not found").into());
    };

    let members = chat_members::Entity::find()
        .filter(chat_members::Column::ChatUuid.eq(uuid))
        .find_with_related(users::Entity)
        .all(&db.0)
        .await
        .context("Failed to fetch chat members")?
        .into_iter()
        .filter_map(|(_, users)| users.into_iter().next())
        .map(|u| User {
            uuid: u.uuid.to_string(),
            username: u.username,
            nickname: u.nickname,
            avatar: u.avatar,
        })
        .collect::<Vec<_>>();

    Ok(ProtoResp(GetChatResp {
        chat: Some(Chat {
            uuid: uuid.to_string(),
            is_group,
            name: maybe_name.unwrap_or_else(|| "Unnamed".into()),
        }),
        members,
    }))
}

#[post("/verify", data = "<req>")]
pub async fn verify(
    jwt: Jwt,
    req: Proto<VerifyPrivateChat>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<VerifyPrivateChatResp>> {
    let my_uuid = jwt.0.sub;
    let with_uuid =
        Uuid::parse_str(&req.0.with_uuid).context("Invalid UUID format for 'with_uuid'")?;

    if my_uuid == with_uuid {
        return Err(anyhow!("Cannot create a private chat with yourself").into());
    }

    let my_cm = Alias::new("my_cm");
    let other_cm = Alias::new("other_cm");

    let existing = chats::Entity::find()
        .join_as(
            JoinType::InnerJoin,
            RelationDef::from(
                chat_members::Entity::belongs_to(chats::Entity)
                    .from(chat_members::Column::ChatUuid)
                    .to(chats::Column::Uuid),
            )
            .rev(),
            my_cm.clone(),
        )
        .filter(Expr::col((my_cm.clone(), chat_members::Column::UserUuid)).eq(my_uuid))
        .join_as(
            JoinType::InnerJoin,
            RelationDef::from(
                chat_members::Entity::belongs_to(chats::Entity)
                    .from(chat_members::Column::ChatUuid)
                    .to(chats::Column::Uuid),
            )
            .rev(),
            other_cm.clone(),
        )
        .filter(Expr::col((other_cm.clone(), chat_members::Column::UserUuid)).eq(with_uuid))
        .filter(chats::Column::IsGroup.eq(false))
        .select_only()
        .column(chats::Column::Uuid)
        .into_tuple::<Uuid>()
        .one(&db.0)
        .await
        .context("Database query failed")?;

    if let Some(chat_uuid) = existing {
        return Ok(ProtoResp(VerifyPrivateChatResp {
            chat_uuid: chat_uuid.to_string(),
        }));
    }

    let txn = db.0.begin().await.context("Failed to begin transaction")?;

    let chat = chats::ActiveModel {
        is_group: Set(false),
        ..Default::default()
    }
    .insert(&txn)
    .await
    .context("Failed to create chat")?;

    let my_member = chat_members::ActiveModel {
        chat_uuid: Set(chat.uuid),
        user_uuid: Set(my_uuid),
        ..Default::default()
    };
    let other_member = chat_members::ActiveModel {
        chat_uuid: Set(chat.uuid),
        user_uuid: Set(with_uuid),
        ..Default::default()
    };

    chat_members::Entity::insert_many(vec![my_member, other_member])
        .exec(&txn)
        .await
        .context("Failed to insert chat members")?;

    let user_rows = users::Entity::find()
        .filter(users::Column::Uuid.is_in(vec![my_uuid, with_uuid]))
        .all(&txn)
        .await
        .context("Failed to load users")?;

    let mut my_name = None;
    let mut other_name = None;
    for u in user_rows {
        if u.uuid == my_uuid {
            my_name = Some(u.nickname.clone());
        } else if u.uuid == with_uuid {
            other_name = Some(u.nickname.clone());
        }
    }

    let my_name = my_name.context("Current user not found in users table")?;
    let other_name = other_name.context("Other user not found in users table")?;

    let my_chat_name = chat_names::ActiveModel {
        chat_uuid: Set(chat.uuid),
        user_uuid: Set(my_uuid),
        name: Set(other_name),
    };
    let other_chat_name = chat_names::ActiveModel {
        chat_uuid: Set(chat.uuid),
        user_uuid: Set(with_uuid),
        name: Set(my_name),
    };

    chat_names::Entity::insert_many(vec![my_chat_name, other_chat_name])
        .exec(&txn)
        .await
        .context("Failed to insert chat names")?;

    txn.commit().await.context("Failed to commit transaction")?;

    Ok(ProtoResp(VerifyPrivateChatResp {
        chat_uuid: chat.uuid.to_string(),
    }))
}

#[post("/g/create", data = "<req>")]
pub async fn g_create(
    jwt: Jwt,
    req: Proto<CreateGroup>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<CreateGroupResp>> {
    let my_uuid = jwt.0.sub;

    let chat_name = req.0.name.trim();

    if chat_name.is_empty() {
        return Err(anyhow!("Group name cannot be empty").into());
    }

    if chat_name.len() > MAX_CHAT_NAME_LENGTH {
        return Err(anyhow!("Group name cannot exceed {} characters", MAX_CHAT_NAME_LENGTH).into());
    }

    let txn = db.0.begin().await.context("Failed to begin transaction")?;

    let chat = chats::ActiveModel {
        is_group: Set(true),
        ..Default::default()
    }
        .insert(&txn)
        .await
        .context("Failed to create group chat")?;

    let mut members = vec![];

    let mut names = vec![];

    for member_uuid_str in &req.0.member_uuids {
        let member_uuid = Uuid::parse_str(member_uuid_str)
            .context("Invalid UUID format in member_uuids")?;
        members.push(chat_members::ActiveModel {
            chat_uuid: Set(chat.uuid),
            user_uuid: Set(member_uuid),
            ..Default::default()
        });
        names.push(chat_names::ActiveModel {
            chat_uuid: Set(chat.uuid),
            user_uuid: Set(member_uuid),
            name: Set(chat_name.to_string()),
        });
    }

    chat_members::Entity::insert_many(members)
        .exec(&txn)
        .await
        .context("Failed to insert group chat members")?;

    chat_names::Entity::insert_many(names)
        .exec(&txn)
        .await
        .context("Failed to insert group chat members")?;

    txn.commit().await.context("Failed to commit transaction")?;

    Ok(ProtoResp(CreateGroupResp {
        uuid: chat.uuid.to_string(),
    }))
}
