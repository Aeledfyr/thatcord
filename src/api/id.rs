use serde::{Deserialize, Serialize};

/// A discord snowflake id
/// https://discordapp.com/developers/docs/reference#snowflakes
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(transparent)]
pub struct Id(pub(crate) u64);

impl Id {
    /// Gets the timestamp that the snowflake id was created at (unix epoch)
    // TODO: use chrono/time
    pub fn get_timestamp(self) -> u64 {
        (self.0 >> 22) + 1_420_070_400_000
    }
}

impl std::str::FromStr for Id {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Id, Self::Err> {
        s.parse::<u64>().map(Id)
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum IdStringOrInt {
    Integer(u64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match IdStringOrInt::deserialize(deserializer)? {
            IdStringOrInt::Integer(i) => Ok(Id(i)),
            IdStringOrInt::String(s) => s
                .parse::<u64>()
                .map(Id)
                .map_err(|_| serde::de::Error::custom("Expected integer in String format")),
        }
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_id {
    ($name:ident) => {
        #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
        #[serde(transparent)]
        pub struct $name(pub Id);

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

impl_id!(ApplicationId);
impl_id!(GuildId);
impl_id!(RoleId);
impl_id!(UserId);
impl_id!(ChannelId);
impl_id!(MessageId);
impl_id!(EmojiId);
impl_id!(AttachmentId);
impl_id!(IntegrationId);
impl_id!(OverwriteId);
