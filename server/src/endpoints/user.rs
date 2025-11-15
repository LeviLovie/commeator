use sea_orm::ActiveModelTrait;

use super::prelude::*;

const ALLOWER_USERNAME_SPECIAL_CHARS: &str = "_-.";

pub fn routes() -> Vec<rocket::Route> {
    routes![get, list, my, setup, check]
}

#[post("/get", data = "<req>")]
pub async fn get(
    _jwt: Jwt,
    req: Proto<GetUser>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<GetUserResp>> {
    let user_model = Users::find()
        .filter(users::Column::Username.eq(&req.0.username))
        .filter(users::Column::SetupComplete.eq(true))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or(anyhow!("User not found"))?;

    Ok(ProtoResp(GetUserResp {
        user: Some(User {
            uuid: user_model.uuid.to_string(),
            username: user_model.username,
            nickname: user_model.nickname,
            avatar: user_model.avatar,
        }),
    }))
}

#[post("/list")]
pub async fn list(_jwt: Jwt, db: &State<Db>) -> ApiResult<ProtoResp<ListUsersResp>> {
    let user_models = Users::find()
        .filter(users::Column::SetupComplete.eq(true))
        .all(&db.0)
        .await
        .context("Database query failed")?;

    let users = user_models
        .into_iter()
        .map(|user_model| User {
            uuid: user_model.uuid.to_string(),
            username: user_model.username,
            nickname: user_model.nickname,
            avatar: user_model.avatar,
        })
        .collect();

    Ok(ProtoResp(ListUsersResp { users }))
}

#[post("/my")]
pub async fn my(jwt: Jwt, db: &State<Db>) -> ApiResult<ProtoResp<GetUserResp>> {
    let user_model = Users::find()
        .filter(users::Column::Uuid.eq(jwt.0.sub))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or(anyhow!("User not found"))?;

    Ok(ProtoResp(GetUserResp {
        user: Some(User {
            uuid: user_model.uuid.to_string(),
            username: user_model.username,
            nickname: user_model.nickname,
            avatar: user_model.avatar,
        }),
    }))
}

#[post("/check")]
pub async fn check(kratos: Kratos, db: &State<Db>) -> ApiResult<ProtoResp<CheckUserResp>> {
    let user_model = Users::find()
        .filter(users::Column::Email.eq(kratos.email.clone()))
        .one(&db.0)
        .await
        .context("Database query failed")?;

    if user_model.is_none() {
        users::ActiveModel {
            email: Set(kratos.email),
            created_at: Set(chrono::Utc::now().naive_utc()),
            setup_complete: Set(false),
            username: Set("".to_string()),
            nickname: Set("".to_string()),
            avatar: Set("".to_string()),
            ..Default::default()
        }
        .insert(&db.0)
        .await
        .context("Database insert failed")?;
    }

    Ok(ProtoResp(CheckUserResp {
        exists: user_model.is_some(),
    }))
}

#[post("/setup", data = "<req>")]
pub async fn setup(
    jwt: Jwt,
    req: Proto<SetupUser>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<SetupUserResp>> {
    if Users::find()
        .filter(users::Column::Uuid.eq(jwt.0.sub))
        .filter(users::Column::SetupComplete.eq(true))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .is_some()
    {
        return Ok(ProtoResp(SetupUserResp {
            result: SetupUserResult::UsernameTaken.into(),
            user: None,
        }));
    }

    if req.0.username.len() < 3
        || req.0.username.len() > 20
        || req.0.username.to_lowercase() != req.0.username
        || !req
            .0
            .username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || ALLOWER_USERNAME_SPECIAL_CHARS.contains(c))
    {
        println!("Invalid username: {}", req.0.username);
        return Ok(ProtoResp(SetupUserResp {
            result: SetupUserResult::InvalidUsername.into(),
            user: None,
        }));
    }

    if req.0.nickname.len() < 3 || req.0.nickname.len() > 20 {
        return Ok(ProtoResp(SetupUserResp {
            result: SetupUserResult::InvalidNickname.into(),
            user: None,
        }));
    }

    let user_model = Users::find()
        .filter(users::Column::Uuid.eq(jwt.0.sub))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or(anyhow!("User not found"))?;

    let mut avatar = req.0.avatar;
    if avatar.is_empty() {
        avatar = format!(
            "https://gravatar.com/avatar/{:x}",
            md5::compute(user_model.email.as_bytes())
        );
    }
    println!("Avatar set to: {}", avatar);

    let mut active_user_model = user_model.into_active_model();
    active_user_model.username = Set(req.0.username);
    active_user_model.nickname = Set(req.0.nickname);
    active_user_model.avatar = Set(avatar);
    active_user_model.setup_complete = Set(true);
    let user_model = active_user_model
        .update(&db.0)
        .await
        .context("Database update failed")?;

    Ok(ProtoResp(SetupUserResp {
        result: SetupUserResult::Success.into(),
        user: Some(User {
            uuid: user_model.uuid.to_string(),
            username: user_model.username,
            nickname: user_model.nickname,
            avatar: user_model.avatar,
        }),
    }))
}
