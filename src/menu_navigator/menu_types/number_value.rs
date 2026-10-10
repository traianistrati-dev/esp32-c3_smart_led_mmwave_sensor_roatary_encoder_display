
use serde::{Deserialize, Serialize};


use core::sync::atomic::{AtomicU32, Ordering};


#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    #[serde(with = "atomic_u32_serde")]
    v: AtomicU32,
    pub min: u32,
    pub max: u32,
    pub step: u32,
    pub as_float: bool,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.v.load(Ordering::Relaxed) == other.v.load(Ordering::Relaxed)
    }
}

impl Value {
    pub const fn new(min: u32, max: u32, init: u32, step: u32, as_float: bool) -> Self {
        Self { v: AtomicU32::new(init), min, max, step, as_float}
    }

    pub fn get(&self) -> u32 {
        self.v.load(Ordering::Relaxed)
    }

    pub fn get_float(&self) -> f32 {
        self.get() as f32
    }

    pub fn set(&self, x: u32) {
        self.v.store(x.clamp(self.min, self.max), Ordering::Relaxed)
    }

    pub fn inc_step(&self) {
        self.set(self.get().saturating_add(self.step))
    }

    pub fn dec_step(&self) {
        self.set(self.get().saturating_sub(self.step))
    }

}

//------ Serialziation ----------------


pub mod atomic_u32_serde {
    use super::*;
    use serde::{ Deserializer, Serializer};

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
