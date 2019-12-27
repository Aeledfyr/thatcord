use super::id::*;
use super::user::User;
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// A Discord Channel
/// https://discordapp.com/developers/docs/resources/channel#channel-object
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Channel {
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub guild_id: Option<GuildId>,
    pub position: Option<u64>,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<MessageId>,
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    pub owner_id: Option<UserId>,
    pub application_id: Option<ApplicationId>,
    pub parent_id: Option<ChannelId>,
    pub last_pin_timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct PermissionOverwrite {
    /// Either role or user id
    pub id: Id,
    /// "role" or "user"
    #[serde(rename = "type")]
    pub kind: String,
    pub allow: u64,
    pub deny: u64,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    DirectMessage = 1,
    GuildVoice = 2,
    GroupDM = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Message {
    pub id: MessageId,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    #[serde(rename = "type")]
    pub kind: MessageType,

    pub author: Option<User>,
    pub member: Option<super::guild::GuildMember>,

    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,

    pub tts: bool,
    pub pinned: bool,

    pub mentions_everyone: bool,
    pub mentions: Vec<User>, // includes partial member?
    pub mention_roles: Vec<RoleId>,
    pub mention_channels: Vec<ChannelMention>,

    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,

    pub application: Option<ApplicationId>,
    pub webhook_id: Option<Id>,
    pub activity: Option<MessageActivity>,
    pub message_reference: Option<MessageReference>,
    pub nonce: Option<String>,
    pub flags: Option<u8>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    PremiumGuildSubscription = 8,
    PremiumGuildSubscriptionTier1 = 9,
    PremiumGuildSubscriptionTier2 = 10,
    PremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ChannelMention {
    pub id: ChannelId,
    pub guild_id: GuildId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub kind: MessageActivityType,
    pub party_id: Option<String>,
}
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[repr(u8)]
pub enum MessageActivityType {
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MessageReference {
    pub message_id: Option<MessageId>,
    pub channel_id: Option<ChannelId>,
    pub guild_id: Option<GuildId>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Attachment {
    pub id: AttachmentId,
    pub filename: String,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Reaction {
    pub count: u64,
    pub current_user: bool,
    pub emoji: Emoji,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Emoji {
    pub id: Option<EmojiId>,
    pub name: Option<String>,
    pub roles: Option<Vec<RoleId>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Embed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<u64>,

    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EmbedThumbnail {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EmbedImage {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EmbedAuthor {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

pub async fn get_channel(token: &str, id: ChannelId) -> Result<Channel> {
    super::api_get(token, &format!("/channels/{}", id), None).await
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ChannelUpdate {
    pub name: Option<String>,
    pub position: Option<u64>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub rate_limit_per_user: Option<u64>,
    pub bitrate: Option<u64>,
    pub user_limit: Option<u64>,
    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,
    pub parent_id: Option<ChannelId>,
}
pub async fn modify_channel(token: &str, id: ChannelId, data: ChannelUpdate) -> Result<Channel> {
    super::api_patch(token, &format!("/channels/{}", id), None, &data).await
}
pub async fn delete_channel(token: &str, id: ChannelId) -> Result<()> {
    super::api_delete(token, &format!("/channels/{}", id), None).await
}
pub async fn get_channel_messages(
    token: &str,
    id: ChannelId,
    query: MessagesQuery,
) -> Result<Vec<Message>> {
    let mut map = std::collections::HashMap::new();
    if let Some(limit) = query.limit {
        map.insert("limit", limit.to_string());
    }
    match query.id {
        Some(MsgQueryLocation::Around(id)) => {
            map.insert("around", id.to_string());
        }
        Some(MsgQueryLocation::Before(id)) => {
            map.insert("before", id.to_string());
        }
        Some(MsgQueryLocation::After(id)) => {
            map.insert("after", id.to_string());
        }
        _ => (),
    }
    super::api_get(token, &format!("/channels/{}/messages", id), Some(map)).await
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MessagesQuery {
    pub id: Option<MsgQueryLocation>,
    pub limit: Option<u64>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MsgQueryLocation {
    Around(MessageId),
    Before(MessageId),
    After(MessageId),
}
pub async fn get_message(token: &str, channel: ChannelId, message: MessageId) -> Result<Channel> {
    super::api_get(
        token,
        &format!("/channels/{}/messages/{}", channel, message),
        None,
    )
    .await
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct NewMessage {
    pub content: String,
    pub nonce: Option<u64>,
    pub tts: bool,
    pub file: Option<Vec<u8>>,
    pub embed: Option<Embed>,
}

// TODO: file uploads (requires multipart/form instead of json)
pub async fn send_message(token: &str, channel: ChannelId, message: NewMessage) -> Result<Channel> {
    super::api_post(
        token,
        &format!("/channels/{}/messages", channel),
        None,
        &message
    )
    .await
}

/// the emoji is in the format "name:id" for custom, or unicode characters
pub async fn create_reaction(token: &str, channel: ChannelId, message: MessageId, emoji: String) -> Result<()> {
    super::api_post(
        token,
        &format!("/channels/{}/messages/{}/reactions/{}/@me", channel, message, emoji),
        None,
        &(),
    )
    .await
}

pub async fn delete_own_reaction(token: &str, channel: ChannelId, message: MessageId, emoji: String) -> Result<()> {
    super::api_delete(
        token,
        &format!("/channels/{}/messages/{}/reactions/{}/@me", channel, message, emoji),
        None,
    )
    .await
}
pub async fn delete_user_reaction(token: &str, channel: ChannelId, message: MessageId, user: UserId, emoji: String) -> Result<()> {
    super::api_delete(
        token,
        &format!("/channels/{}/messages/{}/reactions/{}/{}", channel, message, emoji, user),
        None,
    )
    .await
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ReactionQuery {
    pub id: Option<ReactionQueryLocation>,
    pub limit: Option<u64>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReactionQueryLocation {
    Before(UserId),
    After(UserId),
}

pub async fn get_reaction_users(token: &str, channel: ChannelId, message: MessageId, emoji: String, query: ReactionQuery) -> Result<Vec<User>> {
    let mut map = std::collections::HashMap::new();
    if let Some(limit) = query.limit {
        map.insert("limit", limit.to_string());
    }
    match query.id {
        Some(ReactionQueryLocation::Before(id)) => {
            map.insert("before", id.to_string());
        }
        Some(ReactionQueryLocation::After(id)) => {
            map.insert("after", id.to_string());
        }
        _ => (),
    }
    super::api_get(
        token,
        &format!("/channels/{}/messages/{}/reactions/{}", channel, message, emoji),
        None,
    )
    .await
}
pub async fn delete_all_reactions(token: &str, channel: ChannelId, message: MessageId) -> Result<()> {
    super::api_delete(
        token,
        &format!("/channels/{}/messages/{}/reactions", channel, message),
        None,
    )
    .await
}
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct EditMessage {
    pub content: Option<String>,
    pub embed: Option<Embed>,
    pub flags: Option<u64>,
}
pub async fn edit_message(token: &str, channel: ChannelId, message: MessageId, edit_message: EditMessage) -> Result<Message> {
    super::api_patch(
        token,
        &format!("/channels/{}/messages/{}", channel, message),
        None,
        &edit_message
    )
    .await
}

pub async fn delete_message(token: &str, channel: ChannelId, message: MessageId) -> Result<()> {
    super::api_delete(
        token,
        &format!("/channels/{}/messages/{}", channel, message),
        None,
    )
    .await
}

pub async fn bulk_delete_message(token: &str, channel: ChannelId, messages: Vec<MessageId>) -> Result<()> {
    super::api_post(
        token,
        &format!("/channels/{}/messages/bulk-delete", channel),
        None,
        &serde_json::json! {{ "messages": messages }},
    )
    .await
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct EditChannelPermission {
    pub allow: u64,
    pub deny: u64,
    #[serde(rename = "type")]
    pub kind: String, // "member" or "role"
}
pub async fn edit_channel_permission(token: &str, channel: ChannelId, overwrite: OverwriteId, data: EditChannelPermission) -> Result<()> {
    super::api_patch(
        token,
        &format!("/channels/{}/permissions/{}", channel, overwrite),
        None,
        &data
    )
    .await
}
pub async fn delete_channel_permission(token: &str, channel: ChannelId, overwrite: OverwriteId) -> Result<()> {
    super::api_delete(
        token,
        &format!("/channels/{}/permissions/{}", channel, overwrite),
        None,
    )
    .await
}

pub async fn get_channel_invites(token: &str, channel: ChannelId) -> Result<Vec<super::guild::Invite>> {
    super::api_get(token, &format!("/channels/{}/invites", channel), None).await
}
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateChannelInvite {
    pub max_age: Option<u64>,
    pub max_uses: Option<u64>,
    pub temporary: Option<bool>,
    pub unique: Option<bool>,
}
pub async fn create_channel_invite(token: &str, channel: ChannelId, data: CreateChannelInvite) -> Result<super::guild::Invite> {
    super::api_post(token, &format!("/channels/{}/invites", channel), None, &data).await
}

pub async fn trigger_typing_indicator(token: &str, channel: ChannelId) -> Result<()> {
    super::api_post(token, &format!("/channels/{}/typing", channel), None, &()).await
}

pub async fn get_pinned_messages(token: &str, channel: ChannelId) -> Result<Vec<Message>> {
    super::api_get(token, &format!("/channels/{}/pins", channel), None).await
}
pub async fn pin_message(token: &str, channel: ChannelId, message: MessageId) -> Result<()> {
    super::api_put(token, &format!("/channels/{}/pins/{}", channel, message), None, &()).await
}
pub async fn unpin_message(token: &str, channel: ChannelId, message: MessageId) -> Result<()> {
    super::api_delete(token, &format!("/channels/{}/pins/{}", channel, message), None).await
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct GroupDMAdd {
    pub access_token: String,
    pub nick: String,
}
pub async fn group_dm_add(token: &str, channel: ChannelId, user: UserId, data: GroupDMAdd) -> Result<()> {
    super::api_put(token, &format!("/channels/{}/recipients/{}", channel, user), None, &data).await
}
pub async fn group_dm_remove(token: &str, channel: ChannelId, user: UserId) -> Result<()> {
    super::api_delete(token, &format!("/channels/{}/recipients/{}", channel, user), None).await
}

pub async fn list_guild_emojis(token: &str, guild: GuildId) -> Result<Vec<Emoji>> {
    super::api_get(token, &format!("/guilds/{}/emojis", guild), None).await
}
pub async fn get_guild_emoji(token: &str, guild: GuildId, emoji: EmojiId) -> Result<Emoji> {
    super::api_get(token, &format!("/guilds/{}/emojis/{}", guild, emoji), None).await
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct CreateEmoji {
    pub name: String,
    pub image: String,
    pub roles: Vec<RoleId>,
}

pub async fn create_guild_emoji(token: &str, guild: GuildId, emoji: CreateEmoji) -> Result<Emoji> {
    super::api_post(token, &format!("/guilds/{}/emojis", guild), None, &emoji).await
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct UpdateEmoji {
    pub name: String,
    pub roles: Vec<RoleId>,
}

pub async fn update_guild_emoji(
    token: &str,
    guild: GuildId,
    emoji: EmojiId,
    data: UpdateEmoji,
) -> Result<Emoji> {
    super::api_post(
        token,
        &format!("/guilds/{}/emojis/{}", guild, emoji),
        None,
        &data,
    )
    .await
}
pub async fn delete_guild_emoji(token: &str, guild: GuildId, emoji: EmojiId) -> Result<()> {
    super::api_delete(token, &format!("/guilds/{}/emojis/{}", guild, emoji), None).await
}
