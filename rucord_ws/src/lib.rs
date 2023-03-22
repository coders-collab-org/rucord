#[macro_use]

mod macros;

pub mod error;
pub mod identify_queue;
pub mod shard_bucket;
pub mod websocket;
pub mod websocket_manager;
pub mod websocket_shard;
pub mod websocket_worker;

pub use error::*;
pub use identify_queue::*;
pub use shard_bucket::*;
pub use websocket::*;
pub use websocket_manager::*;
pub use websocket_shard::*;
pub use websocket_worker::*;

pub use rucord_api_types as api_types;

pub type Result<T> = core::result::Result<T, WebSocketError>;
