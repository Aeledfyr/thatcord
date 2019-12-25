use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(transparent)]
pub struct UserId(super::Id);

/// A Discord User
/// https://discordapp.com/developers/docs/resources/user#user-object
#[derive(Serialize, Deserialize, Debug, Clone)]
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
