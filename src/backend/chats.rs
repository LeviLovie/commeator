use dioxus::prelude::*;

#[cfg(feature = "server")]
mod server_utils {
    pub use crate::backend::server_utils::*;
}
#[cfg(feature = "server")]
use server_utils::*;

#[post("/api/chats/list")]
pub async fn list_chats(jwt: String) -> Result<(), ServerFnError> {
    let user = verify_jwt(&jwt).await?;
    info!("Listing chats for user {}", user.email);

    Ok(())
}
