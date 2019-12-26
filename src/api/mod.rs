//! Raw bindings for the discord API

/*
Id
    All ids
Guild
    Guilds
    Members
    Roles
    Activity
User
    User
Channel
    Channel
    Message
    Attachment
    Reaction
    Emoji
    Embed
*/

pub mod id;
pub mod guild;
pub mod user;
pub mod channel;
pub mod gateway;

#[cfg(test)]
mod tests;