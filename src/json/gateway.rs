use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A gateway payload
/// https://discordapp.com/developers/docs/topics/gateway#payloads
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Payload {
    pub op: u8,
    pub d: Value,
    pub s: Option<u64>,
    pub t: Option<String>,
}
