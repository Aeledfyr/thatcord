use crate::api::gateway;
use crate::errors::Result;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use websocket_lite::{AsyncClient, AsyncNetworkStream, Message};
type WSClient = AsyncClient<Box<dyn AsyncNetworkStream + Send + Sync + Unpin>>;

pub(crate) struct Client {
    pub send: tokio::sync::mpsc::UnboundedSender<gateway::Payload>,
    receive: tokio::sync::mpsc::UnboundedReceiver<gateway::Payload>,
}

impl Client {
    pub fn send(&mut self, op: gateway::GatewayOpcode, value: serde_json::Value) -> Result<()> {
        self.send.send(gateway::Payload {
            op: op as u8,
            d: value,
            t: None,
            s: None,
        })?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Option<gateway::Payload> {
        self.receive.recv().await
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
    if payload.op == gateway::GatewayOpcode::Identify as u8 {
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
