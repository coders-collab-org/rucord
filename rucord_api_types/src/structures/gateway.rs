use serde::{Deserialize, Serialize};

/// Represents a gateway URL that can be used for connecting to the Discord Gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayObject {
    /// WSS URL that can be used for connecting to the Gateway
    pub url: String,
}

/// Represents a gateway URL and recommended shard information for connecting to the Discord Gateway with a bot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayBotObject {
    /// WSS URL that can be used for connecting to the Gateway
    pub url: String,

    /// Recommended number of shards to use when connecting
    pub shards: u64,

    /// Information on the current session start limit
    pub session_start_limit: SessionStartLimitObject,
}

/// Represents information on the current session start limit for the Discord Gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStartLimitObject {
    /// Total number of session starts the current user is allowed
    pub total: u64,

    /// Remaining number of session starts the current user is allowed
    pub remaining: u64,

    /// Number of milliseconds after which the limit resets
    pub reset_after: u64,

    /// Number of identify requests allowed per 5 seconds
    pub max_concurrency: u64,
}
