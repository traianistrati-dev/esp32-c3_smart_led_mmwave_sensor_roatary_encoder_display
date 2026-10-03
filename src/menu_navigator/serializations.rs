use serde::{Deserialize, Deserializer, Serialize, Serializer};
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

pub mod atomic_bool_serde {
    use super::*;

    pub fn serialize<S: Serializer>(value: &AtomicBool, serializer: S) -> Result<S::Ok, S::Error>
    {
        value.load(Ordering::Relaxed).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<AtomicBool, D::Error>
    {
        let value = bool::deserialize(deserializer)?;
        Ok(AtomicBool::new(value))
    }
}


pub mod atomic_u32_serde {
    use super::*;

    pub fn serialize<S: Serializer>(value: &AtomicU32, serializer: S) -> Result<S::Ok, S::Error>
    {
        value.load(Ordering::Relaxed).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<AtomicU32, D::Error>
    {
        let value = u32::deserialize(deserializer)?;
        Ok(AtomicU32::new(value))
    }
}


