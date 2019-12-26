use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
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

/// The opcode for a gateway event
/// https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes
#[derive(Copy, Clone, Debug)]
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[non_exhaustive]
pub(crate) enum GatewayOpcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    StatusUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
}

impl std::convert::TryFrom<u8> for GatewayOpcode {
    type Error = crate::errors::Errors;
    fn try_from(op: u8) -> Result<GatewayOpcode, Self::Error> {
        match op {
            0 => Ok(GatewayOpcode::Dispatch),
            1 => Ok(GatewayOpcode::Heartbeat),
            2 => Ok(GatewayOpcode::Identify),
            3 => Ok(GatewayOpcode::StatusUpdate),
            4 => Ok(GatewayOpcode::VoiceStateUpdate),
            6 => Ok(GatewayOpcode::Resume),
            7 => Ok(GatewayOpcode::Reconnect),
            8 => Ok(GatewayOpcode::RequestGuildMembers),
            9 => Ok(GatewayOpcode::InvalidSession),
            10 => Ok(GatewayOpcode::Hello),
            11 => Ok(GatewayOpcode::HeartbeatAck),
            op => Err(crate::errors::Errors::GatewayUnknownOpcode { opcode: op })
        }
    }
}