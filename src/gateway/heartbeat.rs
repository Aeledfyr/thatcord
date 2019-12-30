use crate::api::gateway;
use tokio::sync::watch;

pub struct HeartbeatSender {
    pub last_seq: watch::Sender<Option<u64>>,
    pub last_ack: watch::Sender<std::time::Instant>,
}

#[derive(Clone)]
pub struct HeartbeatHandler {
    pub last_seq: watch::Receiver<Option<u64>>,
    last_send: std::time::Instant,
    last_ack: watch::Receiver<std::time::Instant>,
    disconnect: tokio::sync::mpsc::UnboundedSender<bool>,
}

impl HeartbeatHandler {
    pub fn new(
        disconnect_tx: tokio::sync::mpsc::UnboundedSender<bool>,
    ) -> (HeartbeatHandler, HeartbeatSender) {
        let start = std::time::Instant::now();
        let (last_ack_tx, last_ack_rx) = watch::channel::<std::time::Instant>(start);
        let (last_seq_tx, last_seq_rx) = watch::channel::<Option<u64>>(None);
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
        (handler, sender)
    }
}

pub async fn heartbeat(
    client: tokio::sync::mpsc::UnboundedSender<gateway::Payload>,
    heartbeat_interval: u64,
    mut handler: HeartbeatHandler,
) {
    let mut interval =
        tokio::time::interval(tokio::time::Duration::from_millis(heartbeat_interval));

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
            op: gateway::GatewayOpcode::Heartbeat as u8,
            d: value,
            s: None,
            t: None,
        }) {
            log::error!("Cannot send heartbeat to Discord: {}", e);
        }
    }
}
