

use serde::{Deserialize, Serialize};
use core::sync::atomic::{ AtomicU8, Ordering};

type DataTypeAtomic = AtomicU8;
type DataTypePrimitive = u8;

////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueTime {
    #[serde(with = "atomic_u8_serde")]
    hour: DataTypeAtomic,

    #[serde(with = "atomic_u8_serde")]
    minute: DataTypeAtomic
}



impl ValueTime {
    pub const fn new() -> Self{

        Self{hour: DataTypeAtomic::new(0), minute: DataTypeAtomic::new(0)}
    }

    pub fn new_from(hour: DataTypePrimitive, minute: DataTypePrimitive) -> Self{

        Self{hour: DataTypeAtomic::new(hour), minute: DataTypeAtomic::new(minute)}
    }

    pub fn get(&self) -> (DataTypePrimitive, DataTypePrimitive) {
        (self.hour.load(Ordering::Relaxed), self.minute.load(Ordering::Relaxed))
    }

    pub fn set(&self, hour: DataTypePrimitive, minute: DataTypePrimitive) {
        self.hour.store(hour, Ordering::Relaxed);
        self.minute.store(minute, Ordering::Relaxed);
    }

    pub fn set_minute(&self, minute: DataTypePrimitive) {
        self.minute.store(minute, Ordering::Relaxed);
    }

    pub fn set_hour(&self, hour: DataTypePrimitive) {
        self.hour.store(hour, Ordering::Relaxed);
    }


    pub fn inc_hour(&self) {
        let (hour, _) = self.get();
        if hour < 23 {
            self.set_hour(hour.saturating_add(1))
        }
    }

    pub fn inc_minute(&self) {
        let (_, min) = self.get();
        if min < 58{
            self.set_minute(min.saturating_add(1))
        }
    }

    pub fn dec_hour(&self) {
        let (hour, _) = self.get();
        self.set_hour(hour.saturating_sub(1))
    }

    pub fn dec_minute(&self) {
        let (_, min) = self.get();
        self.set_minute(min.saturating_sub(1))
    }



}
// ------------ == ------------------------------------------

impl PartialEq for ValueTime {
    fn eq(&self, other: &Self) -> bool {
        self.hour.load(Ordering::Relaxed) == other.hour.load(Ordering::Relaxed)
    }
}

//------------- Serialization -------------------------------


pub mod atomic_u8_serde {
    use super::*;
    use serde::{ Deserializer, Serializer};


    pub fn serialize<S: Serializer>(value: &DataTypeAtomic, serializer: S) -> Result<S::Ok, S::Error>
    {
        value.load(Ordering::Relaxed).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<DataTypeAtomic, D::Error>
    {
        let value = DataTypePrimitive::deserialize(deserializer)?;
        Ok(DataTypeAtomic::new(value))
    }
}