use serde::{Deserialize, Serialize};

use crate::UserFlags;

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let flags = u64::deserialize(deserializer)?;

        UserFlags::from_bits(flags)
            .ok_or_else(|| serde::de::Error::custom(format!("Unexpected flags value {}", flags)))
    }
}

impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bits().serialize(serializer)
    }
}
