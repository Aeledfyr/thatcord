use crate::api::gateway::Payload;
use serde_json::Error as JsonError;
use snafu::Snafu;
use tokio::sync::watch::error::SendError as TokioWatchSendError;
use url::ParseError as UrlParseError;
use websocket_lite::Error as WebSocketError;
use surf::Exception as HttpError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) type InternalResult<T, E = Errors> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub struct Error(Errors);

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Errors {
    #[snafu(display("Error serializing to JSON"))]
    JsonSerializationError { source: JsonError },

    #[snafu(display("Error deserializing from JSON: {}", json))]
    JsonDeserializationError { source: JsonError, json: String },

    #[snafu(display("Error converting to JSON value"))]
    JsonConversionError { source: JsonError },

    #[snafu(display("Error connecting to Discord's API"))]
    ApiHttpError { source: DebugWrapper<HttpError> },
    
    #[snafu(display("Cannot send payload to Discord's gateway"))]
    GatewaySendPayloadError {
        source: WebSocketError,
        payload: Payload,
    },

    #[snafu(display("Unknown opcode received from Discord's gateway: {}", opcode))]
    GatewayUnknownOpcode { opcode: u8 },

    #[snafu(display("Invalid opcode to recieve from Discord's gateway: {}", opcode))]
    GatewayInvalidRecieveOpcode { opcode: u8 },

    #[snafu(display("Cannot connect to Discord's gateway"))]
    GatewayConnectError,

    #[snafu(display("Cannot create a client for gateway connection"))]
    GatewayClientBuildError { source: UrlParseError },

    #[snafu(display("Gateway received invalid response: {}", what))]
    GatewayInvalidResponseError { what: String },

    #[snafu(display("Gateway received unknown event: {}", event))]
    GatewayUnknownEvent { event: String },

    #[snafu(display("Gateway couldn't update heartbeat sequenve"))]
    GatewayHeartbeatSeqUpdateError {
        source: TokioWatchSendError<Option<u64>>,
    },

    #[snafu(display("Event handler has returned an error"))]
    EventError,
}

pub(crate) struct DebugWrapper<T: std::fmt::Display>(pub T);
impl<T> std::fmt::Debug for DebugWrapper<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<T> std::fmt::Display for DebugWrapper<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
impl<T> std::error::Error for DebugWrapper<T> where T: std::error::Error {}
impl<T> std::ops::Deref for DebugWrapper<T> where T: std::fmt::Display {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<T> std::ops::DerefMut for DebugWrapper<T> where T: std::fmt::Display {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}