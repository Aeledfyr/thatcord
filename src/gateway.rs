use crate::api::gateway;
use crate::errors::{GatewayError, Result};
use async_trait::async_trait;
use serde_json::json;
use websocket_lite::ClientBuilder;

mod heartbeat;
mod socket;

use heartbeat::{heartbeat, HeartbeatHandler, HeartbeatSender};
use socket::{Client, Connection};

#[async_trait(?Send)]
pub(crate) trait EventHandler {
    async fn handle(&mut self, event: String, data: serde_json::Value) -> Result<()>;
}

enum DiscordState {
    Initial,
    Ready,
}

use gateway::GatewayOpcode;

pub(crate) struct Gateway<F>
where
    F: EventHandler,
{
    token: String,
    client: Client,
    session_id: String,
    state: DiscordState,
    event_handler: F,
    socket: Option<Connection>,

    heartbeat_sender: HeartbeatSender,
    heartbeat_handler: HeartbeatHandler,
}

impl<F> Gateway<F>
where
    F: EventHandler,
{
    async fn ev_ready(&mut self, payload: &serde_json::Value) -> Result<()> {
        self.state = DiscordState::Ready;
        self.session_id = payload["session_id"]
            .as_str()
            .expect("Ready has no session_id?")
            .to_owned();

        log::info!("Discord connection ready!");
        Ok(())
    }

    async fn op0_dispatch(&mut self, payload: gateway::Payload) -> Result<()> {
        self.heartbeat_sender.last_seq.broadcast(payload.s)?;

        let event = payload.t.expect("OP0 does not have event name??");
        let data = payload.d;

        match event.as_str() {
            "READY" => self.ev_ready(&data).await?,
            _ => {}
        }

        self.event_handler.handle(event, data).await
    }

    async fn op1_heartbeat(&mut self, _payload: gateway::Payload) -> Result<()> {
        let last_seq = &*self.heartbeat_handler.last_seq.borrow();
        self.client
            .send(GatewayOpcode::Heartbeat, serde_json::to_value(last_seq)?)
    }

    async fn op7_reconnect(&mut self, _payload: gateway::Payload) -> Result<()> {
        unimplemented!("Discord Gateway Reconnect")
    }
    async fn op9_invalid_session(&mut self, _payload: gateway::Payload) -> Result<()> {
        unimplemented!("Discord Gateway Invalid Session")
    }

    async fn op10_hello(&mut self, payload: gateway::Payload) -> Result<()> {
        if let Some(heartbeat_interval) = payload.d["heartbeat_interval"].as_u64() {
            let sender = self.client.send.clone();
            let handler = self.heartbeat_handler.clone();

            log::trace!("heartbeat interval: {} ms", heartbeat_interval);
            tokio::spawn(async move {
                heartbeat(sender, heartbeat_interval, handler).await;
            });

            self.client.send(
                GatewayOpcode::Identify,
                json!({
                    "token": self.token,
                    "properties": {
                        "$os": std::env::consts::OS,
                        "$browser": crate::LIBRARY_IDENTITY,
                        "$device": crate::LIBRARY_IDENTITY
                    }
                }),
            )
        } else {
            Err(GatewayError::InvalidResponseError {
                what: "Hello does not have heartbeat_interval".to_owned(),
            }
            .into())
        }
    }

    async fn op11_heartbeat_ack(&mut self, _payload: gateway::Payload) -> Result<()> {
        // If a client does not receive a heartbeat ack between its attempts at
        // sending heartbeats, it should immediately terminate the connection
        // with a non-1000 close code, reconnect, and attempt to resume.
        //  ~ https://discordapp.com/developers/docs/topics/gateway#connecting-to-the-gateway
        self.heartbeat_sender
            .last_ack
            .broadcast(std::time::Instant::now())
            .map_err(crate::errors::DiscordError::HeartbeatTimeError)?;
        Ok(())
    }

    pub async fn new(gateway: &str, token: &str, event_handler: F) -> Result<Self> {
        let mut builder = ClientBuilder::new(&format!("{}?v=6&encoding=json", gateway))
            .map_err(GatewayError::from)?;

        builder.add_header(
            "User-Agent".to_owned(),
            crate::discord::USER_AGENT.to_owned(),
        );

        if let Ok(client) = builder.async_connect().await {
            let (disconnect_tx, disconnect_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();
            let (connection, client) = Connection::new(client, disconnect_rx);
            let (handler, sender) = HeartbeatHandler::new(disconnect_tx);

            Ok(Gateway {
                token: token.to_owned(),
                client,
                socket: Some(connection),
                event_handler,

                session_id: "".to_owned(),
                state: DiscordState::Initial,

                heartbeat_sender: sender,
                heartbeat_handler: handler,
            })
        } else {
            Err(GatewayError::ConnectError.into())
        }
    }

    pub(crate) async fn handle(&mut self) -> Result<()> {
        let socket = self.socket.take().expect("Handle can only be run once");
        socket.run();
        while let Some(payload) = self.client.receive().await {
            self.handle_payload(payload).await?;
        }
        Ok(())
    }

    async fn handle_payload(&mut self, payload: gateway::Payload) -> Result<()> {
        use std::convert::TryFrom;

        match GatewayOpcode::try_from(payload.op)? {
            GatewayOpcode::Dispatch => self.op0_dispatch(payload).await,
            GatewayOpcode::Heartbeat => self.op1_heartbeat(payload).await,
            GatewayOpcode::Reconnect => self.op7_reconnect(payload).await,
            GatewayOpcode::InvalidSession => self.op9_invalid_session(payload).await,
            GatewayOpcode::Hello => self.op10_hello(payload).await,
            GatewayOpcode::HeartbeatAck => self.op11_heartbeat_ack(payload).await,
            _ => Err(GatewayError::UnknownOpcode { opcode: payload.op }.into()),
        }
    }
}
