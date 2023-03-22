#[macro_use]
mod macros;

pub type Snowflake = String;

pub mod gateway;
pub mod structures;
pub mod routes;

pub use gateway::*;
pub use structures::*;

