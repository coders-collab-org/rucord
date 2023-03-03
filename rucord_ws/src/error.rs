use async_tungstenite::tungstenite::{protocol::CloseFrame, Error as TungsteniteError};
use derive_more::{Display, Error, From};
use rucord_rest::reqwest::Error as RegError;
use serde_json::Error as JsonError;

#[derive(Debug, From, Error, Display)]
pub enum WebSocketError {
    #[display(fmt = "{_0}")]
    Request(RegError),
    #[display(fmt = "{_0}")]
    Shard(ShardError),
    #[display(fmt = "There are only {_0} sessions available, \
        which is not enough to spawn {_1} shards.")]
    NotEnoughSessionsRemaining(u64, u64),
    #[display(fmt = "{_0}")]
    Json(JsonError),
}

#[derive(Debug, Error, From, Display)]
pub enum ShardError {
    #[display(fmt = "attempting to establish a connection with a non-idle shard")]
    NotIdle,
    #[display(fmt = "{_0}")]
    Tungstenite(TungsteniteError),
    #[display(
        fmt = "{}",
        "_0.as_ref()
        .map_or_else(|| \"Gateway Closed without reason\".into(),
        |e| format!(\"Gateway Closed: {}({})\", e.code, e.reason))"
    )]
    Closed(#[error(not(source))] Option<CloseFrame<'static>>),
}
