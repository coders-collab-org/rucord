use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnavailableGuildObject {
    id: Snowflake,

    unavailable: bool,
}
