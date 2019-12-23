use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
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
