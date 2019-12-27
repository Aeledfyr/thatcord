use super::id::*;
use crate::Result;
use serde::{Deserialize, Serialize};

/// A Discord User
/// https://discordapp.com/developers/docs/resources/user#user-object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub discriminator: String,

    pub avatar: Option<String>,
    pub locale: Option<String>,
    pub email: Option<String>,
    pub flags: Option<u16>,
    pub premium_type: Option<u8>,

    #[serde(default)]
    pub bot: bool,
    #[serde(default)]
    pub system: bool,
    #[serde(default)]
    pub mfa_enabled: bool,
    #[serde(default)]
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Connection {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub revoked: bool,
    pub integrations: Vec<Integration>,
    pub verified: bool,
    pub friend_sync: bool,
    pub show_activity: bool,
    pub visibility: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Integration {
    pub id: IntegrationId,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub enabled: bool,
    pub syncing: bool,
    pub role_id: Id,
    pub expire_behavior: u64,
    pub expire_grace_period: u64,
    pub user: User,
    pub account: IntegrationAccount,
    pub synced_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

pub async fn get_current_user(token: &str) -> Result<User> {
    super::api_get(token, "/users/@me", None).await
}
pub async fn get_user(token: &str, id: UserId) -> Result<User> {
    super::api_get(token, &format!("/users/{}", id), None).await
}
pub async fn get_user_guilds(token: &str) -> Result<Vec<super::guild::Guild>> {
    super::api_get(token, "/users/@me/guilds", None).await
}
pub async fn get_user_dms(token: &str) -> Result<Vec<super::channel::Channel>> {
    super::api_get(token, "/users/@me/channels", None).await
}
pub async fn get_user_connections(token: &str) -> Result<Vec<Connection>> {
    super::api_get(token, "/users/@me/connections", None).await
}

pub async fn leave_guild(token: &str, guild: GuildId) -> Result<()> {
    super::api_delete(token, &format!("/users/@me/guilds/{}", guild), None).await
}

pub async fn create_dm(token: &str, recipient: UserId) -> Result<super::channel::Channel> {
    super::api_post(
        token,
        "/users/@me/channels",
        None,
        &serde_json::json! {{ "recipient_id": recipient }},
    )
    .await
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateGroupDM {
    pub access_tokens: String,
    pub name: String,
}
pub async fn create_group_dm(token: &str, recipient: UserId) -> Result<super::channel::Channel> {
    super::api_post(
        token,
        "/users/@me/channels",
        None,
        &serde_json::json! {{ "recipient_id": recipient }},
    )
    .await
}
// Avatar: encoded as a data uri
pub async fn modify_user(
    token: &str,
    new_name: Option<String>,
    new_avatar: Option<String>,
) -> Result<super::channel::Channel> {
    let mut map = std::collections::HashMap::new();
    if let Some(name) = new_name {
        map.insert("username", name);
    }
    if let Some(avatar) = new_avatar {
        map.insert("avatar", avatar);
    }
    super::api_patch(token, "/users/@me/channels", None, &map).await
}
