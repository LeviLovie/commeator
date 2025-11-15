use super::prelude::*;

pub fn routes() -> Vec<rocket::Route> {
    routes![version]
}

#[post("/version")]
pub fn version() -> ProtoResp<VersionRes> {
    ProtoResp(VersionRes {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
