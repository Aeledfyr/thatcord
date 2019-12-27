use super::channel::*;
use super::guild::*;
use super::id::*;
use super::user::*;

#[test]
fn test_role() {
    assert_eq!(
        serde_json::from_str::<Role>(
            r#"{
        "id": "41771983423143936",
        "name": "WE DEM BOYZZ!!!!!!",
        "color": 3447003,
        "hoist": true,
        "position": 1,
        "permissions": 66321471,
        "managed": false,
        "mentionable": false
    }"#
        )
        .unwrap(),
        Role {
            id: RoleId(Id(41771983423143936)),
            name: String::from("WE DEM BOYZZ!!!!!!"),
            color: 3447003,
            hoist: true,
            position: 1,
            permissions: 66321471,
            managed: false,
            mentionable: false,
        }
    );
}

#[test]
fn test_guild() {
    assert_eq!(
        serde_json::from_str::<Guild>(
            r#"{
        "id": "41771983423143937",
        "application_id": null,
        "name": "Discord Developers",
        "icon": "86e39f7ae3307e811784e2ffd11a7310",
        "splash": null,
        "owner_id": "80351110224678912",
        "region": "us-east",
        "afk_channel_id": "42072017402331136",
        "afk_timeout": 300,
        "embed_enabled": true,
        "embed_channel_id": "41771983444115456",
        "verification_level": 1,
        "default_message_notifications": 0,
        "explicit_content_filter": 0,
        "mfa_level": 0,
        "widget_enabled": false,
        "widget_channel_id": "41771983423143937",
        "roles": [],
        "emojis": [],
        "features": ["INVITE_SPLASH"],
        "unavailable": false
    }"#
        )
        .unwrap(),
        Guild {
            id: GuildId(Id(41771983423143937)),
            application_id: None,
            name: Some(String::from("Discord Developers")),
            icon: Some(String::from("86e39f7ae3307e811784e2ffd11a7310")),
            splash: None,
            owner_id: Some(UserId(Id(80351110224678912))),
            region: Some(String::from("us-east")),
            afk_channel_id: Some(ChannelId(Id(42072017402331136))),
            afk_timeout: Some(300),
            embed_enabled: true,
            embed_channel_id: Some(ChannelId(Id(41771983444115456))),
            verification_level: Some(1),
            default_message_notifications: Some(0),
            explicit_content_filter: Some(0),
            mfa_level: Some(0),
            widget_enabled: false,
            widget_channel_id: Some(ChannelId(Id(41771983423143937))),
            roles: vec![],
            emojis: vec![],
            features: vec![String::from("INVITE_SPLASH")],
            unavailable: Some(false),

            banner: None,
            channels: None,
            description: None,
            discriminator: None,
            joined: None,
            large: false,
            max_members: None,
            max_presences: None,
            member_count: None,
            members: None,
            owner: false,
            permissions: None,
            preferred_locale: None,
            premium_subscription_count: None,
            premium_tier: None,
            presences: None,
            system_channel_id: None,
            vanity_url_code: None,
            voice_states: None,
        }
    );
}