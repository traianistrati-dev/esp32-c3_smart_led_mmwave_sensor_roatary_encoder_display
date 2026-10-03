use core::ops::Range;

use embassy_embedded_hal::adapter::BlockingAsync;
use embedded_storage::nor_flash::MultiwriteNorFlash;
use sequential_storage::cache::{Cache, Uncached};
use sequential_storage::map::{MapConfig, MapStorage, PostcardValue};
use serde::{Deserialize, Serialize};

/// Trebuie să fie identic cu partiția `cfg` din partitions.csv (multiplu de 4 KB, minim 2 sectoare).
pub const CFG_RANGE: Range<u32> = 0x3F0000..0x3F4000;
const KEY_CONFIG: u8 = 1;
pub const CONFIG_VERSION: u8 = 1;


use super::menu::{ModeValues, SensorValues};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Data{
    version: u8
    , pub normal: ModeValues
    , pub night: ModeValues
    , pub sensor: SensorValues

}


impl Default for Data {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION

            , normal: super::menu::NORMAL.clone()
            , night: super::menu::NIGHT.clone()
            , sensor: super::menu::SENSOR.clone()

        }
    }
}

// Marker: sequential-storage serializează automat prin postcard.
impl<'a> PostcardValue<'a> for Data {}

/// Buffer aliniat la 4 bytes (cuvântul flash-ului ESP32).
#[repr(align(4))]
struct AlignedBuf([u8; 128]);

/// Cache fără memorie RAM extra (config-ul se citește rar).
type NoCache = Cache<Uncached, Uncached, Uncached, u8>;

/// Generic peste orice flash NOR: pe ESP32-C3 se folosește cu `esp_storage::FlashStorage<'d>`.
pub struct ConfigStore<F: MultiwriteNorFlash> {
    map: MapStorage<u8, BlockingAsync<F>, NoCache>,
}

impl<F: MultiwriteNorFlash> ConfigStore<F> {
    pub fn new(flash: F) -> Self {
        Self {
            map: MapStorage::new(
                BlockingAsync::new(flash),
                MapConfig::new(CFG_RANGE),
                Cache::new_uncached(),
            ),
        }
    }

    pub async fn load(&mut self) -> Data {
        let mut buf = AlignedBuf([0; 128]);
        match self.map.fetch_item::<Data>(&mut buf.0, &KEY_CONFIG).await {
            Ok(Some(cfg)) if cfg.version == CONFIG_VERSION => cfg,
            _ => Data::default(), // gol, corupt sau versiune veche
        }
    }

    pub async fn save(&mut self) -> Result<(), sequential_storage::Error<F::Error>> {
        let mut buf = AlignedBuf([0; 128]);
        self.map.store_item(&mut buf.0, &KEY_CONFIG, &Data::default()).await
    }
}
