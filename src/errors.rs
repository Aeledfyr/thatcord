pub type Result<T, E = DiscordError> = std::result::Result<T, E>;

macro_rules! convert_error {
    ($from:ty, $to:ty, $which:ident) => {
        impl From<$from> for $to {
            fn from(e: $from) -> $to {
                <$to>::$which(e)
            }
        }
    };
}

#[derive(Debug)]
pub enum DiscordError {
    JsonError(serde_json::Error),
    WebSocketError(websocket_lite::Error),
    HttpError(surf::Exception),
    IoError(std::io::Error),
    HeartbeatSeqUpdateError(tokio::sync::watch::error::SendError<Option<u64>>),
    SocketThread(tokio::sync::mpsc::error::SendError<crate::api::gateway::Payload>),
    HeartbeatTimeError(tokio::sync::watch::error::SendError<std::time::Instant>),
    ApiError(crate::api::ApiError),
    GatewayError(GatewayError),
}

impl std::fmt::Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::JsonError(ref e) => write!(f, "JSON error: {}", e),
            Self::WebSocketError(ref e) => write!(f, "Web Socket error: {}", e),
            Self::IoError(ref e) => write!(f, "IO error: {}", e),
            Self::HttpError(ref e) => write!(f, "HTTP error: {}", e),
            Self::ApiError(ref e) => write!(f, "Discord API error ({}): {}", e.code, e.message),
            Self::HeartbeatSeqUpdateError(ref e) => {
                write!(f, "Heartbeat sequence update error: {}", e)
            }
            Self::HeartbeatTimeError(ref e) => write!(f, "Heartbeat time update error: {}", e),
            Self::SocketThread(ref e) => write!(f, "Socket thread communcation failiure: {}", e),
            Self::GatewayError(ref e) => write!(f, "Gateway error: {}", e),
        }
    }
}

convert_error!(serde_json::Error, DiscordError, JsonError);
convert_error!(websocket_lite::Error, DiscordError, WebSocketError);
convert_error!(crate::api::ApiError, DiscordError, ApiError);
convert_error!(std::io::Error, DiscordError, IoError);
convert_error!(
    tokio::sync::watch::error::SendError<Option<u64>>,
    DiscordError,
    HeartbeatSeqUpdateError
);
convert_error!(
    tokio::sync::mpsc::error::SendError<crate::api::gateway::Payload>,
    DiscordError,
    SocketThread
);
convert_error!(GatewayError, DiscordError, GatewayError);

#[derive(Debug)]
pub enum GatewayError {
    ClientBuildError(url::ParseError),

    ConnectError,
    UnknownOpcode { opcode: u8 },
    InvalidResponseError { what: String },
    UnknownEvent { event: String },
}

impl std::fmt::Display for GatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ClientBuildError(ref e) => write!(f, "Cannot build client: {}", e),

            Self::ConnectError => write!(f, "Cannot connect"),
            Self::UnknownOpcode { opcode } => write!(f, "Unknown opcode: {}", opcode),
            Self::InvalidResponseError { what } => write!(f, "Invalid response: {}", what),
            Self::UnknownEvent { event } => write!(f, "Unknown event: {}", event),
        }
    }
}

convert_error!(url::ParseError, GatewayError, ClientBuildError);
