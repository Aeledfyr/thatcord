use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use super::id::*;

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
    pub roles: Vec<Role>
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