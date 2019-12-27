use crate::api::gateway;
use crate::errors::{InternalResult as Result, *};
use async_trait::async_trait;
use futures_util::SinkExt;
use serde_json::json;
use snafu::ResultExt;
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::stream::StreamExt;
use tokio::sync::{watch, Mutex};
use tokio::time;
use websocket_lite::{AsyncClient, AsyncNetworkStream, ClientBuilder, Message};

type WSClient = AsyncClient<Box<dyn AsyncNetworkStream + Send + Sync + Unpin>>;

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
    client: Arc<Mutex<WSClient>>,
    session_id: String,
    state: DiscordState,
    event_handler: F,

    heartbeat_seq_channel_send: watch::Sender<Option<u64>>,
    heartbeat_seq_channel_recv: watch::Receiver<Option<u64>>,
}

async fn heartbeat(
    client: Arc<Mutex<WSClient>>,
    heartbeat_interval: u64,
    seq_channel: &mut watch::Receiver<Option<u64>>,
) {
    let mut interval = time::interval(time::Duration::from_millis(heartbeat_interval));

    loop {
        interval.tick().await;

        log::trace!("Sending heartbeat");
        println!("{}", serde_json::to_value(Some(0)).unwrap());
        let value: serde_json::Value = serde_json::to_value(seq_channel.recv().await)
            .expect("heartbeat sequence cannot be transformed into a JSON value");
        if let Err(e) = send(
            client.lock().await.deref_mut(),
            GatewayOpcode::Heartbeat,
            value,
        )
        .await
        {
            log::error!("Cannot send heartbeat to Discord: {}", e);
        }
    }
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
        self.heartbeat_seq_channel_send
            .broadcast(payload.s)
            .context(GatewayHeartbeatSeqUpdateError)?;
        let event = payload.t.expect("OP0 does not have event name??");
        let data = payload.d;

        match event.as_str() {
            "READY" => self.ev_ready(&data).await?,
            _ => {}
        }

        self.event_handler.handle(event, data).await
    }

    async fn op1_heartbeat(&mut self, _payload: gateway::Payload) -> Result<()> {
        let ch = &mut self.heartbeat_seq_channel_recv;
        send(
            &mut *self.client.lock().await,
            GatewayOpcode::Heartbeat,
            serde_json::to_value(ch.recv().await).context(JsonConversionError)?,
        )
        .await
    }

    async fn op7_reconnect(&mut self, _payload: gateway::Payload) -> Result<()> {
        unimplemented!("Discord Gateway Reconnect")
    }
    async fn op9_invalid_session(&mut self, _payload: gateway::Payload) -> Result<()> {
        unimplemented!("Discord Gateway Invalid Session")
    }

    async fn op10_hello(&mut self, payload: gateway::Payload) -> Result<()> {
        if let Some(heartbeat_interval) = payload.d["heartbeat_interval"].as_u64() {
            let client = self.client.clone();
            let mut seq_channel = self.heartbeat_seq_channel_recv.clone();

            log::trace!("heartbeat interval: {} ms", heartbeat_interval);
            tokio::spawn(async move {
                heartbeat(client, heartbeat_interval, &mut seq_channel).await;
            });

            self.send(
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
            .await
        } else {
            Err(Errors::GatewayInvalidResponseError {
                what: "Hello does not have heartbeat_interval".to_owned(),
            })
        }
    }

    async fn op11_heartbeat_ack(&mut self, _payload: gateway::Payload) -> Result<()> {
        // If a client does not receive a heartbeat ack between its attempts at
        // sending heartbeats, it should immediately terminate the connection
        // with a non-1000 close code, reconnect, and attempt to resume.
        //  ~ https://discordapp.com/developers/docs/topics/gateway#connecting-to-the-gateway
        //
        // That looks hard, especially with the background heartbeat thingy, so
        // I'm not going to implement this yet.
        Ok(())
    }

    pub async fn new(gateway: &str, token: &str, event_handler: F) -> Result<Self> {
        let mut builder = ClientBuilder::new(&format!("{}?v=6&encoding=json", gateway))
            .context(GatewayClientBuildError)?;
        builder.add_header(
            "User-Agent".to_owned(),
            crate::discord::USER_AGENT.to_owned(),
        );

        if let Ok(client) = builder.async_connect().await {
            let (tx, rx) = watch::channel::<Option<u64>>(None);
            Ok(Gateway {
                token: token.to_owned(),
                client: Arc::new(Mutex::new(client)),
                event_handler,

                session_id: "".to_owned(),
                state: DiscordState::Initial,

                heartbeat_seq_channel_send: tx,
                heartbeat_seq_channel_recv: rx,
            })
        } else {
            Err(Errors::GatewayConnectError {})
        }
    }

    pub(crate) async fn handle(&mut self) -> Result<()> {
        loop {
            let mut c_lock = self.client.lock().await;
            let payload = c_lock.deref_mut().next().await;

            // Drop the Mutex lock so we can use `client` elsewhere.
            // If this wasn't needed, this `loop { match { ... } }` could've been a `while let`
            std::mem::drop(c_lock);

            match payload {
                Some(payload) => match payload {
                    Ok(payload) => {
                        log::trace!("discord raw payload {:?}", payload);

                        if let Some(data) = payload.as_text() {
                            self.handle_payload(
                                serde_json::from_str(data)
                                    .context(JsonDeserializationError { json: data })?,
                            )
                            .await?;
                        } else {
                            log::error!("Discord weird payload: {:?}", payload);
                        }
                    }
                    Err(e) => log::error!("Discord error: {}", e),
                },
                None => break Ok(()),
            }
        }
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
            _ => Err(Errors::GatewayInvalidRecieveOpcode { opcode: payload.op }),
        }
    }

    async fn send(&mut self, opcode: GatewayOpcode, data: serde_json::Value) -> Result<()> {
        send(self.client.lock().await.deref_mut(), opcode, data).await
    }
}

async fn send(client: &mut WSClient, opcode: GatewayOpcode, data: serde_json::Value) -> Result<()> {
    send_payload(
        client,
        gateway::Payload {
            op: opcode as u8,
            d: data,

            s: None,
            t: None,
        },
    )
    .await
}

async fn send_payload(client: &mut WSClient, payload: gateway::Payload) -> Result<()> {
    let payload_str = serde_json::to_string(&payload).context(JsonSerializationError)?;

    log::trace!("sending gateway payload: {}", payload_str);
    client
        .send(Message::text(payload_str))
        .await
        .context(GatewaySendPayloadError { payload })
}
