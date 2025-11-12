pub mod general;
pub mod user;

mod prelude {
    pub use anyhow::{anyhow, Context};
    pub use rocket::{get, post, routes, State};
    pub use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

    pub use crate::{
        db::Db,
        entities::prelude::*,
        entities::*,
        error::ApiResult,
        extractors::{
            jwt::Jwt,
            kratos::Kratos,
            protobuf::{Proto, ProtoResp},
        },
    };
    pub use proto::*;
}
