use crate::api::gateway;
use crate::errors::{GatewayError, Result};
use async_trait::async_trait;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use serde_json::json;
use tokio::sync::watch;
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
    client: Client,
    session_id: String,
    state: DiscordState,
    event_handler: F,
    socket: Option<Connection>,

    heartbeat_sender: HeartbeatSender,
    heartbeat_handler: HeartbeatHandler,
}

pub(crate) struct Client {
    pub send: tokio::sync::mpsc::UnboundedSender<gateway::Payload>,
    pub receive: tokio::sync::mpsc::UnboundedReceiver<gateway::Payload>,
}

impl Client {
    fn send(&mut self, op: gateway::GatewayOpcode, value: serde_json::Value) -> Result<()> {
        self.send.send(gateway::Payload {
            op: op as u8,
            d: value,
            t: None,
            s: None,
        })?;
        Ok(())
    }
}

pub(crate) struct Connection {
    socket: futures_util::stream::Fuse<WSClient>,
    send: tokio::sync::mpsc::UnboundedSender<gateway::Payload>, // Linked to Client.receive
    receive: futures_util::stream::Fuse<tokio::sync::mpsc::UnboundedReceiver<gateway::Payload>>, // Linked to Client.send
    disconnect: futures_util::stream::Fuse<tokio::sync::mpsc::UnboundedReceiver<bool>>, // Linked to Client.send
}

impl Connection {
    /// Creates a new Connection structure, which will be used once
    /// to start the socket handler, and a Client structure, which
    /// holds the queues for communication.
    pub(crate) fn new(
        socket: WSClient,
        disconnect: tokio::sync::mpsc::UnboundedReceiver<bool>,
    ) -> (Self, Client) {
        let (client_send, receive) = tokio::sync::mpsc::unbounded_channel();
        let (send, client_receive) = tokio::sync::mpsc::unbounded_channel();

        (
            Self {
                send,
                receive: receive.fuse(),
                socket: socket.fuse(),
                disconnect: disconnect.fuse(),
            },
            Client {
                send: client_send,
                receive: client_receive,
            },
        )
    }

    /// Starts the socket handling loop, and immediately returns
    /// The handler is run through a spawned future (tokio::spawn)
    pub(crate) fn run(mut self) {
        tokio::spawn(async move {
            loop {
                futures_util::select! {
                    disconnect = self.disconnect.select_next_some() => {
                        if disconnect {
                            self.socket.send(Message::close(Some((4000, String::new())))).await?;
                            self.socket.close().await?;
                            log::error!("Disconnecting from gateway");
                            break;
                        }
                    }
                    payload = self.socket.select_next_some() => {
                        match payload {
                            Ok(payload) => {
                                log::trace!("discord raw payload {:?}", payload);

                                if let Some(data) = payload.as_text() {
                                    let payload: gateway::Payload = serde_json::from_str::<gateway::Payload>(data)?;
                                    self.send.send(payload)?;
                                } else {
                                    log::error!("Discord weird payload: {:?}", payload);
                                }
                            }
                            Err(e) => log::error!("Discord error: {}", e),
                        }
                    }
                    payload = self.receive.select_next_some() => {
                        send(payload, &mut self.socket).await?;
                    }
                }
            }
            Ok::<(), crate::errors::DiscordError>(())
        });
    }
}

async fn send(
    mut payload: gateway::Payload,
    socket: &mut futures_util::stream::Fuse<WSClient>,
) -> Result<()> {
    let payload_str = serde_json::to_string(&payload)?;

    // Don't log the discord token
    if payload.op == GatewayOpcode::Identify as u8 {
        if let Some(token) = payload.d.get_mut("token") {
            *token = serde_json::json!("<TOKEN REDACTED>");
        }
        log::trace!(
            "sending gateway payload: {}",
            serde_json::to_string(&payload)?
        );
    } else {
        log::trace!("sending gateway payload: {}", payload_str);
    }
    socket.send(Message::text(payload_str)).await?;
    Ok(())
}

struct HeartbeatSender {
    last_seq: watch::Sender<Option<u64>>,
    last_ack: watch::Sender<std::time::Instant>,
}

#[derive(Clone)]
struct HeartbeatHandler {
    last_seq: watch::Receiver<Option<u64>>,
    last_send: std::time::Instant,
    last_ack: watch::Receiver<std::time::Instant>,
    disconnect: tokio::sync::mpsc::UnboundedSender<bool>,
}

async fn heartbeat(
    client: tokio::sync::mpsc::UnboundedSender<gateway::Payload>,
    heartbeat_interval: u64,
    mut handler: HeartbeatHandler,
) {
    let mut interval = time::interval(time::Duration::from_millis(heartbeat_interval));

    loop {
        interval.tick().await;

        if *handler.last_ack.borrow() < handler.last_send {
            // Break connection
            log::error!("No response for heartbeat, reconnecting");
            handler.disconnect.send(true).unwrap();
            break;
        }

        handler.last_send = std::time::Instant::now();

        log::trace!("Sending heartbeat");
        let value: serde_json::Value = serde_json::to_value(*handler.last_seq.borrow())
            .expect("heartbeat sequence cannot be transformed into a JSON value");
        if let Err(e) = client.send(gateway::Payload {
            op: GatewayOpcode::Heartbeat as u8,
            d: value,
            s: None,
            t: None,
        }) {
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
            })?
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
            .map_err(|e| GatewayError::from(e))?;

        builder.add_header(
            "User-Agent".to_owned(),
            crate::discord::USER_AGENT.to_owned(),
        );

        if let Ok(client) = builder.async_connect().await {
            let start = std::time::Instant::now();
            let (last_ack_tx, last_ack_rx) = watch::channel::<std::time::Instant>(start);
            let (last_seq_tx, last_seq_rx) = watch::channel::<Option<u64>>(None);
            let (disconnect_tx, disconnect_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();
            let handler = HeartbeatHandler {
                last_ack: last_ack_rx,
                last_seq: last_seq_rx,
                last_send: start,
                disconnect: disconnect_tx,
            };
            let sender = HeartbeatSender {
                last_ack: last_ack_tx,
                last_seq: last_seq_tx,
            };
            let (connection, client) = Connection::new(client, disconnect_rx);
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
            Err(GatewayError::ConnectError)?
        }
    }

    pub(crate) async fn handle(&mut self) -> Result<()> {
        let socket = self.socket.take().expect("Handle can only be run once");
        socket.run();
        while let Some(payload) = self.client.receive.recv().await {
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
            _ => Err(GatewayError::UnknownOpcode { opcode: payload.op })?,
        }
    }
}
