mod discord;
mod errors;
mod gateway;
mod api;

pub mod events;

pub use discord::Discord;
pub use errors::{Error, Result};
pub use api::{guild::Guild, user::User};

const LIBRARY_IDENTITY: &str = "Thatcord";
