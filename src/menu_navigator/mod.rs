
pub mod rotary_encoder;
pub mod sttings;
pub mod menu;
pub mod flash_storage;
pub mod serializations;



// use serde::{Deserialize, Deserializer, Serialize, Serializer};
// use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

// pub mod atomic_bool_serde {
    // use super::*;

    // pub fn serialize<S>(
        // value: &AtomicBool,
        // serializer: S,
    // ) -> Result<S::Ok, S::Error>
    // where
    // S: Serializer,
    // {
        // value.load(Ordering::Relaxed).serialize(serializer)
    // }

    // pub fn deserialize<'de, D>(
        // deserializer: D,
    // ) -> Result<AtomicBool, D::Error>
    // where
    // D: Deserializer<'de>,
    // {
        // let value = bool::deserialize(deserializer)?;
        // Ok(AtomicBool::new(value))
    // }
// }
