use async_tungstenite::tungstenite::{protocol::CloseFrame, Error as TungsteniteError};
use rucord_rest::reqwest::Error as RegError;
use serde_json::Error as JsonError;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum WebSocketError {
    Request(RegError),
    Shard(ShardError),
    NotEnoughSessionsRemaining(u64, u64),
    Json(JsonError),
}

#[derive(Debug)]
pub enum ShardError {
    NotIdle,
    Tungstenite(TungsteniteError),
    Closed(Option<CloseFrame<'static>>),
}

impl From<RegError> for WebSocketError {
    fn from(value: RegError) -> Self {
        Self::Request(value)
    }
}

impl From<ShardError> for WebSocketError {
    fn from(value: ShardError) -> Self {
        Self::Shard(value)
    }
}

impl From<TungsteniteError> for ShardError {
    fn from(value: TungsteniteError) -> Self {
        Self::Tungstenite(value)
    }
}

impl From<JsonError> for WebSocketError {
    fn from(value: JsonError) -> Self {
        Self::Json(value)
    }
}

impl Display for WebSocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebSocketError::Request(e) => write!(f, "{e}"),
            WebSocketError::Shard(e) => write!(f, "{e}"),
            WebSocketError::NotEnoughSessionsRemaining(remaining, shard_count) => write!(f, "There are only {remaining} sessions available, which is not enough to spawn {shard_count} shards."),
            WebSocketError::Json(e) => write!(f, "{e}")

        }
    }
}

impl Display for ShardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShardError::NotIdle => write!(
                f,
                "Attempting to establish a connection with a non-idle shard"
            ),
            ShardError::Tungstenite(e) => write!(f, "{e}"),

            // TODO: Closed message.
            ShardError::Closed(Some(e)) => write!(f, "Gateway Closed: {}({})", e.code, e.reason),
            ShardError::Closed(None) => write!(f, "Gateway Closed without reason"),
        }
    }
}

impl Error for WebSocketError {}
impl Error for ShardError {}
