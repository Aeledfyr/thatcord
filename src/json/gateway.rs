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
