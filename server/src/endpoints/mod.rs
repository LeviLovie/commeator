pub mod chat;
pub mod general;
pub mod jwt;
pub mod message;
pub mod user;

mod prelude {
    pub use anyhow::{Context, anyhow};
    pub use rocket::{State, post, routes};
    pub use sea_orm::{
        ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, JoinType, QueryFilter,
        QuerySelect, RelationDef, Set, TransactionTrait, prelude::Expr, prelude::Uuid,
        sea_query::Alias, sqlx::types::chrono::Utc,
    };

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
