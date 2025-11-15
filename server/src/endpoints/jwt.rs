use chrono::Duration;

use super::prelude::*;
use crate::jwt;

pub fn routes() -> Vec<rocket::Route> {
    routes![verify, generate]
}

#[post("/verify", data = "<req>")]
pub async fn verify(
    _kratos: Kratos,
    req: Proto<VerifyJwt>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<VerifyJwtResp>> {
    match jwt::verify(req.0.token.clone()) {
        jwt::JwtStatus::Expired => Ok(ProtoResp(VerifyJwtResp { valid: false })),
        jwt::JwtStatus::Invalid => Ok(ProtoResp(VerifyJwtResp { valid: false })),
        jwt::JwtStatus::Valid(_claims) => {
            match Devices::find()
                .filter(devices::Column::Jwt.eq(req.0.token.clone()))
                .one(&db.0)
                .await
                .context("Database query failed")?
            {
                Some(_) => Ok(ProtoResp(VerifyJwtResp { valid: true })),
                None => Ok(ProtoResp(VerifyJwtResp { valid: false })),
            }
        }
    }
}

#[post("/gen", data = "<req>")]
pub async fn generate(
    kratos: Kratos,
    req: Proto<GenJwt>,
    db: &State<Db>,
) -> ApiResult<ProtoResp<GenJwtResp>> {
    let user = Users::find()
        .filter(users::Column::Email.eq(kratos.email.clone()))
        .one(&db.0)
        .await
        .context("Database query failed")?
        .ok_or_else(|| anyhow!("User not found"))?;

    let expiration = Utc::now() + Duration::hours(24 * 7);
    let claims = jwt::Claims {
        sub: user.uuid,
        exp: expiration.timestamp(),
    };
    let token = jwt::create(&claims).context("Failed to create JWT")?;

    devices::ActiveModel {
        user_uuid: Set(user.uuid),
        jwt: Set(token.clone()),
        os: Set(req.0.os.clone()),
        ..Default::default()
    }
    .insert(&db.0)
    .await
    .context("Failed to save device JWT")?;

    Ok(ProtoResp(GenJwtResp {
        token,
        expires_at: expiration.timestamp().to_string(),
    }))
}
