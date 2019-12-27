use super::id::*;
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// See the following official documentation for item descriptions.
/// https://discordapp.com/developers/docs/resources/guild#guild-object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Guild {
    pub id: GuildId,
    pub unavailable: Option<bool>,

    // The rest will only be created whenever Discord sends us a GUILD_CREATE event
    pub name: Option<String>,
    pub discriminator: Option<String>,
    pub region: Option<String>,

    pub afk_timeout: Option<u64>,

    pub mfa_level: Option<u8>,
    pub premium_tier: Option<u8>,
    pub verification_level: Option<u8>,
    pub explicit_content_filter: Option<u8>,
    pub default_message_notifications: Option<u8>,

    pub member_count: Option<u64>,

    pub owner_id: Option<UserId>,
    pub application_id: Option<String>,

    pub afk_channel_id: Option<ChannelId>,
    pub embed_channel_id: Option<ChannelId>,
    pub system_channel_id: Option<ChannelId>,
    pub widget_channel_id: Option<ChannelId>,

    pub joined: Option<String>, // TODO: ISO8601 parse

    pub icon: Option<String>,
    pub splash: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub vanity_url_code: Option<String>,
    pub preferred_locale: Option<String>,

    pub permissions: Option<u64>,

    pub premium_subscription_count: Option<u64>,

    pub max_presences: Option<u64>,
    pub max_members: Option<u64>,

    #[serde(default)]
    pub features: Vec<String>,

    #[serde(default)]
    pub owner: bool,

    #[serde(default)]
    pub embed_enabled: bool,

    #[serde(default)]
    pub widget_enabled: bool,

    #[serde(default)]
    pub large: bool,

    pub presences: Option<Vec<PresenceUpdate>>,
    pub channels: Option<Vec<super::channel::Channel>>,
    pub members: Option<Vec<GuildMember>>,
    pub voice_states: Option<Vec<VoiceState>>,

    pub emojis: Vec<super::channel::Emoji>,
    pub roles: Vec<Role>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct GuildMember {
    pub user: super::user::User,
    pub nick: Option<String>,
    pub roles: Vec<RoleId>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub color: u64,
    pub hoist: bool,
    pub position: u64,
    pub permissions: u64,
    pub managed: bool,
    pub mentionable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct PresenceUpdate {
    pub user: serde_json::Value,
    pub roles: Vec<RoleId>,
    pub game: Option<Activity>,
    pub guild_id: GuildId,
    pub status: OnlineStatus,
    pub activities: Vec<Activity>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OnlineStatus {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "dnd")]
    Dnd,
    #[serde(rename = "offline")]
    Offline,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ClientStatus {
    pub desktop: Option<OnlineStatus>,
    pub mobile: Option<OnlineStatus>,
    pub web: Option<OnlineStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ActivityType,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<Vec<ActivityTimestamp>>,
    pub application_id: Option<ApplicationId>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<super::channel::Emoji>,

    pub party: Option<serde_json::Value>,
    pub assets: Option<serde_json::Value>,
    pub secrets: Option<serde_json::Value>,

    pub instance: Option<bool>,
    pub flags: Option<u64>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Custom = 4,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ActivityTimestamp {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct VoiceState {
    pub guild_id: Option<GuildId>,
    pub channel_id: Option<ChannelId>,
    pub user_id: UserId,
    pub member: Option<GuildMember>,
    pub session_id: String,
    pub deaf: bool,
    pub mute: bool,
    pub self_deaf: bool,
    pub self_mute: bool,
    pub self_stream: Option<bool>,
    pub suppress: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Invite {
    pub code: String,
    pub guild: Option<Guild>,
    pub channel: Option<super::channel::Channel>,
    pub target_user: Option<super::user::User>,
    pub target_user_type: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub approximate_member_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct InviteMetadata {
    pub inviter: super::user::User,
    pub uses: u64,
    pub max_uses: u64,
    pub max_age: u64,
    pub temporary: bool,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct VoiceRegion {
    pub id: String,
    pub name: String,
    pub vip: bool,
    pub optimal: bool,
    pub deprecated: bool,
    pub custom: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Ban {
    pub reason: Option<String>,
    pub user: UserId,
}

pub async fn get_invite(token: &str, invite: &str) -> Result<Invite> {
    super::api_get(token, &format!("/invites/{}", invite), None).await
}

pub async fn delete_invite(token: &str, invite: &str) -> Result<()> {
    super::api_delete(token, &format!("/invites/{}", invite), None).await
}

// begin guild prune
// create guild integration
// modify guild integration
// delete guild integration
// sync guild integration
// modify guild embed
// get guild widget image

pub async fn get_guild(token: &str, guild: GuildId) -> Result<Guild> {
    super::api_get(token, &format!("/guilds/{}", guild), None).await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateGuild {
    pub name: String,
    pub region: String,
    pub icon: String,
    pub verification_level: u64,
    pub default_message_notifications: u64,
    pub explicit_content_filter: u64,
    pub roles: Vec<Role>,
    pub channels: Vec<super::channel::Channel>, // TODO: without id?
}
pub async fn create_guild(token: &str, data: CreateGuild) -> Result<Guild> {
    super::api_post(token, "/guilds", None, &data).await
}
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ModifyGuild {
    pub name: Option<String>,
    pub region: Option<String>,
    pub verification_level: Option<u64>,
    pub default_message_notifications: Option<u64>,
    pub explicit_content_filter: Option<u64>,
    pub afk_channel_id: Option<ChannelId>,
    pub afk_timeout: Option<u64>,
    pub owner_id: Option<UserId>,
    pub system_channel_id: Option<ChannelId>,

    // TODO: images
    pub splash: Option<String>,
    pub banner: Option<String>,
    pub icon: Option<String>,
}
pub async fn modify_guild(token: &str, guild: GuildId, data: ModifyGuild) -> Result<Guild> {
    super::api_patch(token, &format!("/guilds/{}", guild), None, &data).await
}
pub async fn delete_guild(token: &str, guild: GuildId) -> Result<()> {
    super::api_delete(token, &format!("/guilds/{}", guild), None).await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateChannel {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: Option<super::channel::ChannelType>,
    pub topic: Option<String>,
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,
    pub position: Option<u64>,
    pub permission_overwrites: Option<Vec<super::channel::PermissionOverwrite>>,
    pub parent_id: Option<ChannelId>,
    pub nsfw: Option<bool>,
}
pub async fn create_channel(
    token: &str,
    guild: GuildId,
    data: CreateChannel,
) -> Result<super::channel::Channel> {
    super::api_post(token, &format!("/guilds/{}/channels", guild), None, &data).await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ChannelPosition {
    pub id: ChannelId,
    pub position: u64,
}
pub async fn modify_channel_order(
    token: &str,
    guild: GuildId,
    data: Vec<ChannelPosition>,
) -> Result<super::channel::Channel> {
    super::api_patch(token, &format!("/guilds/{}/channels", guild), None, &data).await
}

pub async fn get_guild_channels(
    token: &str,
    guild: GuildId,
) -> Result<Vec<super::channel::Channel>> {
    super::api_get(token, &format!("/guilds/{}/channels", guild), None).await
}
pub async fn list_guild_members(token: &str, guild: GuildId) -> Result<Vec<GuildMember>> {
    super::api_get(token, &format!("/guilds/{}/members", guild), None).await
}
pub async fn get_guild_member(token: &str, guild: GuildId, user: UserId) -> Result<GuildMember> {
    super::api_get(token, &format!("/guilds/{}/members/{}", guild, user), None).await
}
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct GuildInvite {
    pub access_token: String,
    pub nick: Option<String>,
    pub roles: Option<Vec<RoleId>>,
    pub mute: Option<bool>,
    pub deaf: Option<bool>,
}
pub async fn add_guild_member(
    token: &str,
    guild: GuildId,
    user: UserId,
    data: GuildInvite,
) -> Result<GuildMember> {
    super::api_put(
        token,
        &format!("/guilds/{}/members/{}", guild, user),
        None,
        &data,
    )
    .await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ModifyMember {
    pub nick: Option<String>,
    pub roles: Option<Vec<RoleId>>,
    pub mute: Option<bool>,
    pub deaf: Option<bool>,
    // Disconnects someone from the channel if it is Null
    pub channel_id: Optional<ChannelId>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Optional<T> {
    Hide,
    Null,
    Some(T),
}
impl<T> Serialize for Optional<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Optional::Some(ref value) => serializer.serialize_some(value),
            Optional::Hide => serializer.serialize_none(),
            Optional::Null => serializer.serialize_unit(),
        }
    }
}
pub async fn modify_guild_member(
    token: &str,
    guild: GuildId,
    user: UserId,
    data: ModifyMember,
) -> Result<()> {
    super::api_put(
        token,
        &format!("/guilds/{}/members/{}", guild, user),
        None,
        &data,
    )
    .await
}

pub async fn modify_current_nick(token: &str, guild: GuildId, nick: String) -> Result<String> {
    super::api_put(
        token,
        &format!("/guilds/{}/members/@me/nick", guild),
        None,
        &serde_json::json! {{ "nick": nick }},
    )
    .await
}

pub async fn add_role_to_member(
    token: &str,
    guild: GuildId,
    user: UserId,
    role: RoleId,
) -> Result<()> {
    super::api_put(
        token,
        &format!("/guilds/{}/members/{}/roles/{}", guild, user, role),
        None,
        &(),
    )
    .await
}
pub async fn remove_role_from_member(
    token: &str,
    guild: GuildId,
    user: UserId,
    role: RoleId,
) -> Result<()> {
    super::api_delete(
        token,
        &format!("/guilds/{}/members/{}/roles/{}", guild, user, role),
        None,
    )
    .await
}
pub async fn remove_member(token: &str, guild: GuildId, user: UserId) -> Result<()> {
    super::api_delete(token, &format!("/guilds/{}/members/{}", guild, user), None).await
}

pub async fn get_bans(token: &str, guild: GuildId) -> Result<Vec<Ban>> {
    super::api_get(token, &format!("/guilds/{}/bans", guild), None).await
}
// TODO: optional, with 404 -> No ban
pub async fn get_user_ban(token: &str, guild: GuildId, user: UserId) -> Result<Ban> {
    super::api_get(token, &format!("/guilds/{}/bans/{}", guild, user), None).await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateBan {
    #[serde(rename = "delete-message-days")]
    pub delete_message_days: Option<u64>,
    pub reason: Option<String>,
}
pub async fn create_ban(token: &str, guild: GuildId, user: UserId, data: CreateBan) -> Result<()> {
    let mut query = std::collections::HashMap::new();
    if let Some(days) = data.delete_message_days {
        query.insert("delete-message-days", days.to_string());
    }
    if let Some(reason) = data.reason {
        query.insert("reason", reason);
    }
    super::api_put(
        token,
        &format!("/guilds/{}/bans/{}", guild, user),
        Some(query),
        &(),
    )
    .await
}
pub async fn remove_ban(token: &str, guild: GuildId, user: UserId) -> Result<()> {
    super::api_delete(token, &format!("/guilds/{}/bans/{}", guild, user), None).await
}

pub async fn get_guild_roles(token: &str, guild: GuildId) -> Result<Vec<Role>> {
    super::api_get(token, &format!("/guilds/{}/roles", guild), None).await
}
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateRole {
    pub name: String,
    pub permissions: u64,
    pub color: u64,
    pub hoist: bool,
    pub mentionable: bool,
}
pub async fn create_role(token: &str, guild: GuildId, role: CreateRole) -> Result<Vec<Role>> {
    super::api_post(token, &format!("/guilds/{}/roles", guild), None, &role).await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct RolePosition {
    pub id: RoleId,
    pub position: u64,
}
pub async fn modify_role_order(
    token: &str,
    guild: GuildId,
    data: Vec<RolePosition>,
) -> Result<Role> {
    super::api_patch(token, &format!("/guilds/{}/roles", guild), None, &data).await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ModifyRole {
    pub name: Option<String>,
    pub permissions: Option<u64>,
    pub color: Option<u64>,
    pub hoist: Option<bool>,
    pub mentionable: Option<bool>,
}
pub async fn modify_role(
    token: &str,
    guild: GuildId,
    role: RoleId,
    data: ModifyRole,
) -> Result<Role> {
    super::api_patch(
        token,
        &format!("/guilds/{}/role/{}", guild, role),
        None,
        &data,
    )
    .await
}
pub async fn delete_role(token: &str, guild: GuildId, role: RoleId) -> Result<()> {
    super::api_delete(token, &format!("/guilds/{}/role/{}", guild, role), None).await
}

pub async fn get_guild_invites(token: &str, guild: GuildId) -> Result<Vec<Role>> {
    super::api_get(token, &format!("/guilds/{}/invites", guild), None).await
}
pub async fn get_guild_integrations(
    token: &str,
    guild: GuildId,
) -> Result<Vec<super::user::Integration>> {
    super::api_get(token, &format!("/guilds/{}/integrations", guild), None).await
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Pruned {
    pub pruned: Option<u64>,
}
pub async fn get_prune_count(token: &str, guild: GuildId, days: Option<u64>) -> Result<Pruned> {
    let mut query = std::collections::HashMap::new();
    if let Some(d) = days {
        query.insert("day", d.to_string());
    }
    super::api_get(token, &format!("/guilds/{}/prune", guild), Some(query)).await
}

pub async fn begin_prune(
    token: &str,
    guild: GuildId,
    days: Option<u64>,
    compute_prune_count: Option<bool>,
) -> Result<Pruned> {
    let mut query = std::collections::HashMap::new();
    if let Some(d) = days {
        query.insert("day", d.to_string());
    }
    if let Some(d) = compute_prune_count {
        query.insert("compute_prune_count", d.to_string());
    }
    super::api_post(token, &format!("/guilds/{}/prune", guild), Some(query), &()).await
}

pub async fn get_voice_regions(token: &str, guild: GuildId) -> Result<Vec<VoiceRegion>> {
    super::api_get(token, &format!("/guilds/{}/regions", guild), None).await
}
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct GuildEmbed {
    pub enabled: bool,
    pub channel_id: Option<ChannelId>,
}
pub async fn get_guild_embed(token: &str, guild: GuildId) -> Result<GuildEmbed> {
    super::api_get(token, &format!("/guilds/{}/embed", guild), None).await
}
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ModifyGuildEmbed {
    pub enabled: Option<bool>,
    pub channel_id: Option<ChannelId>,
}
pub async fn modify_guild_embed(
    token: &str,
    guild: GuildId,
    data: ModifyGuildEmbed,
) -> Result<GuildEmbed> {
    super::api_patch(token, &format!("/guilds/{}/embed", guild), None, &data).await
}
pub async fn get_guild_vanity_url(token: &str, guild: GuildId) -> Result<Invite> {
    super::api_get(token, &format!("/guilds/{}/integrations", guild), None).await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateGuildIntegration {
    pub id: IntegrationId,
    #[serde(rename = "type")]
    pub kind: String,
}
pub async fn create_guild_integration(
    token: &str,
    guild: GuildId,
    data: CreateGuildIntegration,
) -> Result<()> {
    super::api_post(
        token,
        &format!("/guilds/{}/integrations", guild),
        None,
        &data,
    )
    .await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ModifyGuildIntegration {
    pub expire_behavior: u64,
    pub expire_grace_period: u64,
    pub enable_emoticons: bool,
}
pub async fn modify_guild_integration(
    token: &str,
    guild: GuildId,
    integration: IntegrationId,
    data: ModifyGuildIntegration,
) -> Result<()> {
    super::api_patch(
        token,
        &format!("/guilds/{}/integration/{}", guild, integration),
        None,
        &data,
    )
    .await
}

pub async fn delete_guild_integration(
    token: &str,
    guild: GuildId,
    integration: IntegrationId,
) -> Result<()> {
    super::api_delete(
        token,
        &format!("/guilds/{}/integration/{}", guild, integration),
        None,
    )
    .await
}

pub async fn sync_guild_integration(
    token: &str,
    guild: GuildId,
    integration: IntegrationId,
) -> Result<()> {
    super::api_post(
        token,
        &format!("/guilds/{}/integration/{}/sync", guild, integration),
        None,
        &(),
    )
    .await
}

// pub async fn get_guild_widget_image(token: &str, guild: GuildId) -> Result<Vec<super::user::Integration>> {
//     super::api_get(token, &format!("/guilds/{}/integrations", guild)).await
// }
