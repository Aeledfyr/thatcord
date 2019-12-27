mod api;
mod discord;
mod errors;
mod gateway;

pub mod events;

pub use api::{guild::Guild, user::User};
pub use discord::Discord;
pub use errors::{DiscordError, Result};

const LIBRARY_IDENTITY: &str = "Thatcord";
