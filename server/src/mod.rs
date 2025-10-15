pub mod chats;
pub mod jwt;
pub mod messages;
pub mod users;

#[cfg(feature = "server")]
mod verify_kratos;

#[cfg(feature = "server")]
#[allow(unused_imports)]
mod entities;

#[cfg(feature = "server")]
mod server_utils {
    pub use super::entities::prelude::*;
    pub use super::entities::*;
    pub use super::jwt::verify_jwt;
    pub use super::verify_kratos::verify_kratos_cookie;

    pub use sea_orm::{
        sea_query::Query, ActiveModelTrait, ActiveValue::*, ColumnTrait, EntityTrait, JoinType,
        QueryFilter, QuerySelect, RelationTrait,
    };
}
