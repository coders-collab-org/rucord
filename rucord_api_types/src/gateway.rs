#![allow(non_upper_case_globals)]

use std::{env, str::FromStr};

use bitflags::bitflags;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_json::{from_value, Value};
use strum_macros::{Display, EnumString};

use crate::{Snowflake, UserObject};

type JsonMap = serde_json::Map<String, Value>;

/// Represents a Discord gateway opcode.
///
/// [Discord documentation](https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromPrimitive, PartialEq, Eq)]
pub enum GatewayOpcode {
    /// An event was dispatched.
    Dispatch = 0,

    /// Fired periodically by the client to keep the connection alive.
    Heartbeat = 1,

    /// Starts a new session during the initial handshake.
    Identify = 2,

    /// Update the client's presence.
    PresenceUpdate = 3,

    /// Used to join/leave or move between voice channels.
    VoiceStateUpdate = 4,

    /// Resume a previous session that was disconnected.
    Resume = 6,

    /// You should attempt to reconnect and resume immediately.
    Reconnect = 7,

    /// Request information about offline guild members in a large guild.
    RequestGuildMembers = 8,

    /// The session has been invalidated. You should reconnect and identify/resume accordingly.
    InvalidSession = 9,

    /// Sent immediately after connecting, contains the heartbeat_interval to use.
    Hello = 10,

    /// Sent in response to receiving a heartbeat to acknowledge that it has been received.
    HeartbeatAck = 11,
}

/// Represents a Discord gateway close event code and associated error message.
///
/// [Discord documentation](https://discord.com/developers/docs/topics/opcodes-and-status-codes#gateway-gateway-opcodes).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GatewayCloseCode {
    /// We're not sure what went wrong. Try reconnecting?
    UnknownError = 4000,
    /// You sent an invalid Gateway opcode or an invalid payload for an opcode. Don't do that!
    UnknownOpcode = 4001,
    /// You sent an invalid payload to Discord. Don't do that!
    DecodeError = 4002,
    /// You sent us a payload prior to identifying.
    NotAuthenticated = 4003,
    /// The account token sent with your identify payload is incorrect.
    AuthenticationFailed = 4004,
    /// You sent more than one identify payload. Don't do that!
    AlreadyAuthenticated = 4005,
    /// The sequence sent when resuming the session was invalid. Reconnect and start a new session.
    InvalidSeq = 4007,
    /// Woah nelly! You're sending payloads to us too quickly. Slow it down! You will be disconnected on receiving this.
    RateLimited = 4008,
    /// Your session timed out. Reconnect and start a new one.
    SessionTimedOut = 4009,
    /// You sent us an invalid shard when identifying.
    InvalidShard = 4010,
    /// The session would have handled too many guilds - you are required to shard your connection in order to connect.
    ShardingRequired = 4011,
    /// You sent an invalid version for the gateway.
    InvalidApiVersion = 4012,
    /// You sent an invalid intent for a Gateway Intent. You may have incorrectly calculated the bitwise value.
    InvalidIntents = 4013,
    /// You sent a disallowed intent for a Gateway Intent. You may have tried to specify an intent that you have not enabled or are not approved for.
    DisallowedIntents = 4014,
}

bitflags! {
    /// Represents the different events that can be received over the gateway.
    ///
    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway#list-of-intents).
    #[derive(Serialize, Default, Deserialize)]
    pub struct GatewayIntentBits: u64 {
        const Guilds = 1 << 0;
        const GuildMembers = 1 << 1;
        const GuildModeration = 1 << 2;
        const GuildEmojisAndStickers = 1 << 3;
        const GuildIntegrations = 1 << 4;
        const GuildWebhooks = 1 << 5;
        const GuildInvites = 1 << 6;
        const GuildVoiceStates = 1 << 7;
        const GuildPresences = 1 << 8;
        const GuildMessages = 1 << 9;
        const GuildMessageReactions = 1 << 10;
        const GuildMessageTyping = 1 << 11;
        const DirectMessages = 1 << 12;
        const DirectMessageReactions = 1 << 13;
        const DirectMessageTyping = 1 << 14;
        const MessageContent = 1 << 15;
        const GuildScheduledEvents = 1 << 16;
        const AutoModerationConfiguration = 1 << 20;
        const AutoModerationExecution = 1 << 21;
    }
}

/// The events that can be received over the Discord gateway.
///
/// This enum represents all of the different events that can be sent to your bot
/// over the Discord gateway. You can listen for specific events using the `GatewayIntents`
/// feature.
///
/// [Discord documentation](https://discord.com/developers/docs/topics/gateway#commands-and-events-gateway-events).
#[derive(Debug, Clone, Serialize, Deserialize, EnumString, Display, PartialEq, Eq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum GatewayDispatchEvents {
    /// Emitted when the application command permissions for a guild have been updated.
    ApplicationCommandPermissionsUpdate,
    /// Emitted when a new channel is created, relevant to the current user.
    ChannelCreate,
    /// Emitted when a channel is deleted, relevant to the current user.
    ChannelDelete,
    /// Emitted when the pins of a channel are updated, relevant to the current user.
    ChannelPinsUpdate,
    /// Emitted when a channel is updated, relevant to the current user.
    ChannelUpdate,
    /// Emitted when a new user is banned from a guild.
    GuildBanAdd,
    /// Emitted when a user is unbanned from a guild.
    GuildBanRemove,
    /// Emitted when a new guild is available to the client.
    GuildCreate,
    /// Emitted when a guild becomes unavailable to the client.
    GuildDelete,
    /// Emitted when a guild's emojis have been updated.
    GuildEmojisUpdate,
    /// Emitted when a guild integration is updated.
    GuildIntegrationsUpdate,
    /// Emitted when a new member joins a guild.
    GuildMemberAdd,
    /// Emitted when a member leaves a guild, or is kicked.
    GuildMemberRemove,
    /// Emitted in response to Guild Request Members.
    GuildMembersChunk,
    /// Emitted when a guild member is updated.
    GuildMemberUpdate,
    /// Emitted when a new role is created.
    GuildRoleCreate,
    /// Emitted when a role is deleted.
    GuildRoleDelete,
    /// Emitted when a role is updated.
    GuildRoleUpdate,
    /// Emitted when a guild's stickers have been updated.
    GuildStickersUpdate,
    /// Emitted when a guild is updated.
    GuildUpdate,
    /// Emitted when a new integration is created for a guild.
    IntegrationCreate,
    /// Emitted when an integration is deleted from a guild.
    IntegrationDelete,
    /// Emitted when an integration is updated for a guild.
    IntegrationUpdate,
    /// Emitted when an interaction is created.
    InteractionCreate,
    /// Emitted when an invite is created.
    InviteCreate,
    /// Emitted when an invite is deleted.
    InviteDelete,
    /// Emitted when a message is created.
    MessageCreate,

    /// Emitted when a message is deleted.
    MessageDelete,

    /// Emitted when messages are bulk deleted.
    MessageDeleteBulk,

    /// Emitted when a reaction is added to a message.
    MessageReactionAdd,

    /// Emitted when a reaction is removed from a message.
    MessageReactionRemove,

    /// Emitted when all reactions are removed from a message.
    MessageReactionRemoveAll,

    /// Emitted when all reactions of a specific emoji are removed from a message.
    MessageReactionRemoveEmoji,

    /// Emitted when a message is updated.
    MessageUpdate,

    /// Emitted when a user's presence is updated.
    PresenceUpdate,

    /// Emitted when a stage instance is created.
    StageInstanceCreate,

    /// Emitted when a stage instance is deleted.
    StageInstanceDelete,

    /// Emitted when a stage instance is updated.
    StageInstanceUpdate,

    /// Emitted when the client has completed the initial handshake with the gateway.
    Ready,

    /// Emitted after a connection is resumed after a disconnect.
    Resumed,

    /// Emitted when a new thread is created.
    ThreadCreate,

    /// Emitted when a thread is deleted.
    ThreadDelete,

    /// Emitted when gaining access to a channel that is part of a thread.
    ThreadListSync,

    /// Emitted when the thread member object for the current user is updated.
    ThreadMemberUpdate,

    ThreadMembersUpdate,

    /// Emitted when any property of a thread is updated.
    ThreadUpdate,
    /// Emitted when a user starts typing.
    TypingStart,

    /// Emitted when the properties of the current user are updated.
    UserUpdate,

    /// Emitted when a voice server is updated for a guild.
    VoiceServerUpdate,

    /// Emitted when a user joins, leaves, or moves between voice channels.
    VoiceStateUpdate,

    /// Emitted when a guild channel's webhook is created, updated, or deleted.
    WebhooksUpdate,

    /// Emitted when a scheduled event is created for a guild.
    GuildScheduledEventCreate,

    /// Emitted when a scheduled event is updated for a guild.
    GuildScheduledEventUpdate,

    /// Emitted when a scheduled event is deleted for a guild.
    GuildScheduledEventDelete,

    /// Emitted when a user is added to a scheduled event for a guild.
    GuildScheduledEventUserAdd,

    /// Emitted when a user is removed from a scheduled event for a guild.
    GuildScheduledEventUserRemove,

    /// Emitted when an auto-moderation rule is created for a guild.
    AutoModerationRuleCreate,

    /// Emitted when an auto-moderation rule is updated for a guild.
    AutoModerationRuleUpdate,

    /// Emitted when an auto-moderation rule is deleted for a guild.
    AutoModerationRuleDelete,

    /// Emitted when an auto-moderation action is executed for a guild.
    AutoModerationActionExecution,

    /// Emitted when a new audit log entry is added to a guild's audit log.
    GuildAuditLogEntryCreate,
}

#[derive(Debug, Clone, Deserialize)]
pub enum GatewaySendPayload {
    Identify(IdentifyData),
    Resume(ResumeData),
    Heartbeat(Option<u64>),
    RequestGuildMembers(RequestGuildMembersData),
    VoiceStateUpdate(VoiceStateUpdateData),
    UpdatePresence(UpdatePresenceData),
}

#[derive(Debug, Clone, Serialize)]
pub enum GatewayReceivePayload {
    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway-events#hello).
    Hello(u64),

    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway#sending-heartbeats).
    HeartbeatRequest,

    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway#heartbeat).
    HeartbeatAck,

    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway#invalid-session).
    InvalidSession(bool),

    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway#reconnect).
    Reconnect,

    /// [Discord documentation](https://discord.com/developers/docs/topics/gateway#reconnect).
    Dispatch((i64, DispatchPayload)),

    UnknownOp(u64, JsonMap),
}

//TODO: Write all events when need it.
/// Represents a payload for a `Dispatch` GatewayOpcode.
/// [Discord documentation](https://discord.com/developers/docs/topics/gateway-events#receive-events).
#[derive(Debug, Clone, Serialize)]
pub enum DispatchPayload {
    /// Contains the initial state information.
    Ready(ReadyData),
    /// Response to [Resume](https://discord.com/developers/docs/topics/gateway-events#resumed).
    Resume,
    ApplicationCommandPermissionsUpdate(JsonMap),

    AutoModerationRuleCreate(JsonMap),

    AutoModerationRuleUpdate(JsonMap),

    AutoModerationRuleDelete(JsonMap),

    AutoModerationActionExecution(JsonMap),

    ChannelCreate(JsonMap),

    ChannelUpdate(JsonMap),

    ChannelDelete(JsonMap),

    ChannelPinsUpdate(JsonMap),

    ThreadCreate(JsonMap),

    ThreadUpdate(JsonMap),

    ThreadDelete(JsonMap),

    ThreadListSync(JsonMap),

    ThreadMemberUpdate(JsonMap),

    ThreadMembersUpdate(JsonMap),

    GuildCreate(JsonMap),

    GuildUpdate(JsonMap),

    GuildDelete(JsonMap),

    GuildAuditLogEntryCreate(JsonMap),

    GuildBanAdd(JsonMap),

    GuildBanRemove(JsonMap),

    GuildEmojisUpdate(JsonMap),

    GuildStickersUpdate(JsonMap),

    GuildIntegrationsUpdate(JsonMap),

    GuildMemberAdd(JsonMap),

    GuildMemberRemove(JsonMap),

    GuildMemberUpdate(JsonMap),

    GuildMembersChunk(JsonMap),

    GuildRoleCreate(JsonMap),

    GuildRoleUpdate(JsonMap),

    GuildRoleDelete(JsonMap),

    GuildScheduledEventCreate(JsonMap),

    GuildScheduledEventUpdate(JsonMap),

    GuildScheduledEventDelete(JsonMap),

    GuildScheduledEventUserAdd(JsonMap),

    GuildScheduledEventUserRemove(JsonMap),

    InteractionCreate(JsonMap),

    IntegrationUpdate(JsonMap),

    IntegrationDelete(JsonMap),

    InviteCreate(JsonMap),

    InviteDelete(JsonMap),

    MessageCreate(JsonMap),

    MessageUpdate(JsonMap),

    MessageDelete(JsonMap),

    MessageDeleteBulk(JsonMap),

    MessageReactionAdd(JsonMap),

    MessageReactionRemove(JsonMap),

    MessageReactionRemoveAll(JsonMap),

    MessageReactionRemoveEmoji(JsonMap),

    PresenceUpdate(JsonMap),

    StageInstanceCreate(JsonMap),

    StageInstanceUpdate(JsonMap),

    StageInstanceDelete(JsonMap),

    TypingStart(JsonMap),

    UserUpdate(JsonMap),

    VoiceStateUpdate(JsonMap),

    VoiceServerUpdate(JsonMap),

    WebhooksUpdate(JsonMap),

    Unknown(String, JsonMap),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct IdentifyData {
    pub token: String,

    pub properties: IdentifyConnectionProperties,

    #[serde(default)]
    pub compress: Option<bool>,

    #[serde(default)]
    pub large_threshold: Option<u64>,

    #[serde(default)]
    pub shard: Option<(u64, u64)>,

    #[serde(default)]
    pub presence: Option<UpdatePresenceData>,

    pub intents: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentifyConnectionProperties {
    pub os: String,

    pub browser: String,

    pub device: String,
}

impl Default for IdentifyConnectionProperties {
    fn default() -> Self {
        let browser = format!("rucord {}", env!("CARGO_PKG_VERSION"));

        Self {
            os: browser.clone(),
            browser,
            device: env::consts::OS.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeData {
    pub token: String,

    pub session_id: String,

    pub seq: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestGuildMembersData {
    pub guild_id: Snowflake,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    pub limit: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<Snowflake>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceStateUpdateData {
    pub guild_id: Snowflake,

    pub channel_id: Snowflake,

    pub self_mute: bool,

    pub self_deaf: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePresenceData {
    pub since: Option<u64>,

    //TODO: When write ActivityObject.
    pub activities: Vec<Value>,

    pub status: PresenceStateType,

    pub afk: bool,
}

#[derive(Debug, Clone, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
pub enum PresenceStateType {
    Online,
    Dnd,
    Idle,
    Invisible,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadyData {
    pub v: u8,

    pub user: UserObject,

    //TODO: When write UnavailableGuildObject.
    pub guilds: Vec<Value>,

    pub session_id: String,

    pub resume_gateway_url: String,

    #[serde(default)]
    pub shard: Option<(u64, u64)>,

    //TODO: When write ApplicationObject.
    pub application: Value,
}

impl Serialize for GatewaySendPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut obj = serializer.serialize_struct("GatewaySendPayload", 2)?;
        match self {
            GatewaySendPayload::Resume(d) => {
                obj.serialize_field("op", &(GatewayOpcode::Resume as u64))?;
                obj.serialize_field("d", d)?;
            }
            GatewaySendPayload::Identify(d) => {
                obj.serialize_field("op", &(GatewayOpcode::Identify as u64))?;
                obj.serialize_field("d", d)?;
            }
            GatewaySendPayload::Heartbeat(d) => {
                obj.serialize_field("op", &(GatewayOpcode::Heartbeat as u64))?;
                obj.serialize_field("d", d)?;
            }
            GatewaySendPayload::RequestGuildMembers(d) => {
                obj.serialize_field("op", &(GatewayOpcode::RequestGuildMembers as u64))?;
                obj.serialize_field("d", d)?;
            }
            GatewaySendPayload::VoiceStateUpdate(d) => {
                obj.serialize_field("op", &(GatewayOpcode::VoiceStateUpdate as u64))?;
                obj.serialize_field("d", d)?;
            }
            GatewaySendPayload::UpdatePresence(d) => {
                obj.serialize_field("op", &(GatewayOpcode::PresenceUpdate as u64))?;
                obj.serialize_field("d", d)?;
            }
        }

        obj.end()
    }
}

impl GatewayReceivePayload {
    pub fn unpack(str: String) -> Self {
        let mut payload: JsonMap = Value::from_str(&str).and_then(from_value).unwrap();

        let op = to_value!(payload, op);

        let Some(op) = FromPrimitive::from_u64(op) else {
            return Self::UnknownOp(op, payload);
        };

        match op {
            GatewayOpcode::Hello => {
                let mut d: JsonMap = to_value!(payload, d);

                Self::Hello(to_value!(d, heartbeat_interval))
            }
            GatewayOpcode::Heartbeat => Self::HeartbeatRequest,
            GatewayOpcode::HeartbeatAck => Self::HeartbeatAck,
            GatewayOpcode::InvalidSession => Self::InvalidSession(to_value!(payload, d)),
            GatewayOpcode::Reconnect => Self::Reconnect,
            GatewayOpcode::Dispatch => Self::Dispatch(DispatchPayload::from_payload(payload)),
            _ => unreachable!("not receive op"),
        }
    }
}

impl DispatchPayload {
    pub fn from_payload(mut payload: JsonMap) -> (i64, Self) {
        let s = to_value!(payload, s);

        let event_str: String = to_value!(payload, t);

        let Ok(event) = GatewayDispatchEvents::from_str(&event_str) else {
            return (s, Self::Unknown(event_str.into(), payload));
        };

        (
            s,
            match event {
                GatewayDispatchEvents::Ready => Self::Ready(to_value!(payload, d)),
                GatewayDispatchEvents::Resumed => Self::Resume,

                _ => unimplemented!(),
            },
        )
    }
}
