

use serde::{Deserialize, Serialize};
use core::sync::atomic::{ AtomicU8, Ordering};

type DataTypeAtomic = AtomicU8;
type DataTypePrimitive = u8;

use super::bool_value::{ValueBool};

////////////////////////////////////////////////////////////////

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueTime {
    #[serde(with = "atomic_u8_serde")]
    hour: DataTypeAtomic,

    #[serde(with = "atomic_u8_serde")]
    minute: DataTypeAtomic,

    #[serde(with = "atomic_u8_serde")]
    seconds: DataTypeAtomic,

    pub config_min: ValueBool
}



impl ValueTime {
    pub const fn new() -> Self{

        Self{hour: DataTypeAtomic::new(0), minute: DataTypeAtomic::new(0), seconds: DataTypeAtomic::new(0), config_min: ValueBool::new_false()}
    }

    pub fn new_from(hour: DataTypePrimitive, minute: DataTypePrimitive) -> Self{

        Self{hour: DataTypeAtomic::new(hour), minute: DataTypeAtomic::new(minute), seconds: DataTypeAtomic::new(0), config_min: ValueBool::new_false()}
    }

    pub fn get(&self) -> (DataTypePrimitive, DataTypePrimitive, DataTypePrimitive) {
        (self.hour.load(Ordering::Relaxed), self.minute.load(Ordering::Relaxed), self.seconds.load(Ordering::Relaxed))
    }

    pub fn set(&self, hour: DataTypePrimitive, minute: DataTypePrimitive) {
        self.hour.store(hour, Ordering::Relaxed);
        self.minute.store(minute, Ordering::Relaxed);
        self.seconds.store(0, Ordering::Relaxed);
    }

    pub fn set_minute(&self, minute: DataTypePrimitive) {
        self.minute.store(minute, Ordering::Relaxed);
    }

    pub fn set_hour(&self, hour: DataTypePrimitive) {
        self.hour.store(hour, Ordering::Relaxed);
    }

    pub fn set_seconds(&self, seconds: DataTypePrimitive) {
        self.seconds.store(seconds, Ordering::Relaxed);
    }


    pub fn inc(&self) {
        if self.config_min.get(){
            self.inc_minute();
        }else{
            self.inc_hour();
        }
    }

    pub fn dec(&self) {
        if self.config_min.get(){
            self.dec_minute();
        }else{
            self.dec_hour();
        }
    }


    pub fn inc_seconds(&self) {
        let (_, _, seconds) = self.get();
        self.set_seconds((seconds + 1) % 60);

        if ((seconds + 1) % 60) == 0 {
            self.inc_minute();

            if self.get().1 == 0{
                self.inc_hour();
            }
        }

    }

    pub fn inc_hour(&self) {
        let (hour, _, _) = self.get();
        self.set_hour((hour + 1) % 24);
    }

    pub fn dec_hour(&self) {
        let (hour, _, _) = self.get();
        self.set_hour((hour + 23) % 24);
    }

    pub fn inc_minute(&self) {
        let (_, min, _) = self.get();
        self.set_minute((min + 1) % 60)
    }


    pub fn dec_minute(&self) {
        let (_, min, _) = self.get();
        self.set_minute((min + 59) % 60);
    }

    // pub fn toggle_config_minute(&self){
    // self.config_min.set(!self.config_min.get());
    // }



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