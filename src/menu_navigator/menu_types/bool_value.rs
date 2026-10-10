

use serde::{Deserialize, Serialize};
use core::sync::atomic::{ AtomicBool, Ordering};


#[derive(Serialize, Deserialize, Debug)]
pub struct ValueBool {
    #[serde(with = "atomic_bool_serde")]
    v: AtomicBool
}

impl PartialEq for ValueBool {
    fn eq(&self, other: &Self) -> bool {
        self.v.load(Ordering::Relaxed)
        == other.v.load(Ordering::Relaxed)
    }
}


impl ValueBool {
    pub const fn new() -> Self{

        Self{v: AtomicBool::new(true)}
    }
    pub const fn new_false() -> Self{

        Self{v: AtomicBool::new(false)}
    }

    pub fn new_from(b: bool) -> Self{

        Self{v: AtomicBool::new(b)}
    }

    pub fn get(&self) -> bool {
        self.v.load(Ordering::Relaxed)
    }

    pub fn set(&self, x: bool) {
        self.v.store(x, Ordering::Relaxed)
    }

    pub fn toggle_enabled(&self){
        self.set(!self.get());
    }
}

//------------- Serialization -------------------------------


pub mod atomic_bool_serde {
    use super::*;
    use serde::{ Deserializer, Serializer};

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