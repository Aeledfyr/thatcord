use serde::{Deserialize, Serialize};

/// See tbe following official documentation for item descriptions.
/// https://discordapp.com/developers/docs/resources/guild#guild-object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Guild {
    pub id: String,
    pub unavailable: bool,

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

    pub owner_id: Option<String>,
    pub application_id: Option<String>,

    pub afk_channel_id: Option<String>,
    pub embed_channel_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub widget_channel_id: Option<String>,

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
}

// TODO: roles, emojis, voice_states, members, channels
