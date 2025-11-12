use sea_orm::ActiveModelTrait;

use super::prelude::*;

const ALLOWER_USERNAME_SPECIAL_CHARS: &str = "_-.";

pub fn routes() -> Vec<rocket::Route> {
    routes![get, my, setup]
}

#[post("/get", data = "<req>")]
pub async fn get(
    _jwt: Jwt,
    req: Proto<GetUser>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<GetUserResp>> {
    let user_model = Users::find()
        .filter(users::Column::Username.eq(&req.0.username))
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

#[get("/my")]
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

#[get("/setup", data = "<req>")]
pub async fn setup(
    jwt: Jwt,
    kratos: Kratos,
    req: Proto<SetupUser>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<SetupUserResp>> {
    if Users::find()
        .filter(users::Column::Email.eq(&kratos.email))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .is_some()
    {
        return Ok(ProtoResp(SetupUserResp {
            result: 1, // SetupUserResponse::UsernameTaken
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
        return Ok(ProtoResp(SetupUserResp {
            result: 2, // SetupUserResponse::InvalidUsername
            user: None,
        }));
    }

    if req.0.nickname.len() < 3 || req.0.nickname.len() > 20 {
        return Ok(ProtoResp(SetupUserResp {
            result: 3, // SetupUserResponse::InvalidNickname
            user: None,
        }));
    }

    let user_model = users::ActiveModel {
        uuid: Set(jwt.0.sub),
        username: Set(req.0.username),
        nickname: Set(req.0.nickname),
        avatar: Set("".to_string()),
        ..Default::default()
    }
    .insert(&db.0)
    .await
    .context("Database insert failed")?;

    Ok(ProtoResp(SetupUserResp {
        result: 0, // SetupUserResponse::Success
        user: Some(User {
            uuid: user_model.uuid.to_string(),
            username: user_model.username,
            nickname: user_model.nickname,
            avatar: user_model.avatar,
        }),
    }))
}
