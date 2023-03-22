#[macro_use]
mod macros;

pub type Snowflake = String;

pub mod gateway;
pub mod routes;
pub mod structures;

pub use gateway::*;
pub use structures::*;
