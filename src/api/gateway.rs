use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// A gateway payload
/// https://discordapp.com/developers/docs/topics/gateway#payloads
#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub op: u8,
    pub d: Value,
    pub s: Option<u64>,
    pub t: Option<String>,
}

/// The opcode for a gateway event
/// https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway-opcodes
#[derive(Copy, Clone, Debug, Serialize_repr, Deserialize_repr)]
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
    type Error = crate::errors::GatewayError;

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
            op => Err(crate::errors::GatewayError::UnknownOpcode { opcode: op }),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GatewayResponse {
    pub url: String,
}

/// Gets the url for the wss gateway that a client should use when connecting
///
/// https://discordapp.com/developers/docs/topics/gateway#get-gateway
pub(crate) async fn get_gateway() -> Result<GatewayResponse, surf::Exception> {
    let gateway = surf::get(format!("{}/gateway", crate::discord::API_PATH))
        .recv_json()
        .await?;
    Ok(gateway)
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BotGatewayResponse {
    pub url: String,
    pub shards: u32,
    pub session_start_limit: SessionStartLimit,
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SessionStartLimit {
    pub total: u32,
    pub remaining: u32,
    pub reset_after: u32,
}

/// Gets the url for the wss gateway that a bot should use when connecting
///
/// https://discordapp.com/developers/docs/topics/gateway#get-gateway-bot
pub(crate) async fn get_bot_gateway(token: &str) -> Result<BotGatewayResponse, surf::Exception> {
    let gateway = surf::get(format!("{}/gateway/bot", crate::discord::API_PATH))
        .set_header("Authorization", format!("Bot {}", token))
        .set_header("User-Agent", crate::discord::USER_AGENT.to_owned())
        .recv_json()
        .await?;
    Ok(gateway)
}
