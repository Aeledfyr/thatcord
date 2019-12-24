mod discord;
mod errors;
mod gateway;
mod json;

pub mod events;

pub use discord::Discord;
pub use errors::{Error, Result};
pub use json::{Guild, User};

const LIBRARY_IDENTITY: &str = "Thatcord";
