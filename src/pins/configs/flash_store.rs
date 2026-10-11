// <<< GENERATED>>>
// Flash store (from the Configuration tab) — auto-updated; edit it in the tab.
// The bytes the store owns: exactly the default `nvs` partition of the partition table,
// which `verify` below checks on the chip.
pub const STORE_RANGE: core::ops::Range<u32> = 0x9000..0xF000;
// ESP-IDF `data` subtype of that partition: 0x06 undefined, 0x02 nvs.
pub const STORE_SUBTYPE: u8 = 0x02;
// <<< GENERATED END >>>

// Everything below is editable — your changes are preserved on regeneration.
//
// Settings kept in the chip's own flash: `sequential-storage` keeps a small
// key -> value map in STORE_RANGE and spreads the writes over its sectors.
// Put what you want to keep in `Data`, then:
//
//     let mut data = flash_store.load_blocking();      // Async: .load().await
//     data.counter += 1;
//     flash_store.save_blocking(&data).ok();          // Async: .save(&data).await
//
// A save writes flash with interrupts masked for each sector, and an erase
// takes milliseconds: save when something changed, not in a tight loop.

use core::ops::Range;

use embassy_embedded_hal::adapter::BlockingAsync;
use embedded_storage::Storage;
use embedded_storage::nor_flash::NorFlash;
use esp_bootloader_esp_idf::partitions::{self, PARTITION_TABLE_MAX_LEN};
use sequential_storage::cache::{Cache, Uncached};
use sequential_storage::map::{MapConfig, MapStorage, PostcardValue};
use serde::{Deserialize, Serialize};

/// Bump it whenever `Data` changes shape: a value stored by another version
/// is ignored, and `load` returns `Data::default()` instead of misreading it.
pub const CONFIG_VERSION: u8 = 1;
const KEY_DATA: u8 = 1;

/// What the store keeps. Add your own fields - anything serde can derive.
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Data {
    version: u8,
    pub counter: u32,

    //Custom data
    pub normal: crate::menu_navigator::menu::ModeValues
    , pub night: crate::menu_navigator::menu::ModeValues
    , pub sensor: crate::menu_navigator::menu::SensorValues
    , pub light: crate::menu_navigator::menu::LightValues
    // , pub system: crate::menu_navigator::menu::SystemValues
    //Custom data

}

impl Default for Data {
    fn default() -> Self {
        Self {
            version: CONFIG_VERSION,
            counter: 0,
            //Custom data
            normal: crate::menu_navigator::menu::NORMAL.clone()
            , night: crate::menu_navigator::menu::NIGHT.clone()
            , sensor: crate::menu_navigator::menu::SENSOR.clone()
            , light: crate::menu_navigator::menu::LIGHT.clone()
            // , system: crate::menu_navigator::menu::SYSTEM.clone()
            //Custom data
        }
    }
}

impl<'a> PostcardValue<'a> for Data {}

/// The serialized `Data` must fit here (raise it with your fields). Aligned
/// to the flash's 4-byte word.
#[repr(align(4))]
struct Buf([u8; 128]);

type NoCache = Cache<Uncached, Uncached, Uncached, u8>;

/// The store, over any NOR flash - `esp_storage::FlashStorage` here. It owns
/// the flash; call `verify` first if you want the check.
pub struct ConfigStore<F: NorFlash> {
    map: MapStorage<u8, BlockingAsync<F>, NoCache>,
}

impl<F: NorFlash> ConfigStore<F> {
    pub fn new(flash: F) -> Self {
        Self {
            map: MapStorage::new(
                BlockingAsync::new(flash),
                // Checked while building: a range under two sectors, or off a
                // sector boundary, fails the build instead of every boot.
                const { MapConfig::<BlockingAsync<F>>::new(STORE_RANGE) },
                Cache::new_uncached(),
            ),
        }
    }

    /// The stored `Data`, or `Data::default()` when there is none yet (first
    /// boot), it is of another `CONFIG_VERSION`, or it cannot be read.
    pub async fn load(&mut self) -> Data {
        let mut buf = Buf([0; 128]);
        match self.map.fetch_item::<Data>(&mut buf.0, &KEY_DATA).await {
            Ok(Some(d)) if d.version == CONFIG_VERSION => d,
            _ => Data::default(),
        }
    }

    /// Store `data`. When the range holds something sequential-storage cannot
    /// read - another firmware's data, a store that moved - it is erased once
    /// and the write tried again.
    // pub async fn save(&mut self, data: &Data) -> Result<(), sequential_storage::Error<F::Error>> {
    pub async fn save_menu_data(&mut self) -> Result<(), sequential_storage::Error<F::Error>> {
        let data: &Data = &Data::default();

        let mut buf = Buf([0; 128]);
        match self.map.store_item(&mut buf.0, &KEY_DATA, data).await {
            Err(sequential_storage::Error::Corrupted { .. }) => {
                self.map.erase_all().await?;
                self.map.store_item(&mut buf.0, &KEY_DATA, data).await
            }
            other => other,
        }
    }

    pub async fn save(&mut self, data: &Data) -> Result<(), sequential_storage::Error<F::Error>> {

        let mut buf = Buf([0; 128]);
        match self.map.store_item(&mut buf.0, &KEY_DATA, data).await {
            Err(sequential_storage::Error::Corrupted { .. }) => {
                self.map.erase_all().await?;
                self.map.store_item(&mut buf.0, &KEY_DATA, data).await
            }
            other => other,
        }
    }



    /// `load` for the blocking runtimes.
    pub fn load_blocking(&mut self) -> Data {
        embassy_futures::block_on(self.load())
    }

    /// `save` for the blocking runtimes.
    pub fn save_blocking(&mut self, data: &Data) -> Result<(), sequential_storage::Error<F::Error>> {
        embassy_futures::block_on(self.save(data))
    }
}

/// Why `verify` refused.
#[derive(Debug)]
pub enum StoreError {
    /// The partition table could not be read, or failed its MD5 check.
    Table(partitions::Error),
    /// No `data` partition of STORE_SUBTYPE covers exactly STORE_RANGE;
    /// `found` is the first one of that subtype, if any.
    NotReserved { found: Option<Range<u32>> },
    /// The partition is flagged `readonly`. ESP-IDF code would not write it;
    /// esp-storage writes raw flash and ignores the flag, so the store would
    /// overwrite a partition the table marks read-only.
    ReadOnly,
    /// STORE_RANGE ends past the flash size the bootloader header states.
    BeyondFlash { capacity: u32 },
}

/// Is STORE_RANGE really reserved on THIS chip? Reads the partition table the
/// bootloader uses and looks for a `data` partition of STORE_SUBTYPE with
/// exactly that range. Call it before `ConfigStore::new`, which takes `flash`:
///
///     flash_store::verify(&mut flash).expect("flash store not reserved");
///
/// The lookup is raw on purpose: esp-bootloader-esp-idf's `find_partition`
/// panics on a subtype it has no name for, and a table may hold one.
pub fn verify<F: Storage>(flash: &mut F) -> Result<(), StoreError> {
    let capacity = flash.capacity() as u32;
    if capacity != 0 && STORE_RANGE.end > capacity {
        return Err(StoreError::BeyondFlash { capacity });
    }
    let mut buf = [0u8; PARTITION_TABLE_MAX_LEN];
    let table = partitions::read_partition_table(flash, &mut buf).map_err(StoreError::Table)?;
    let mut found = None;
    for e in table.iter() {
        if e.raw_type() != 0x01 || e.raw_subtype() != STORE_SUBTYPE {
            continue;
        }
        let range = e.offset()..e.offset() + e.len();
        if range == STORE_RANGE {
            return if e.flags() & PART_FLAG_READONLY != 0 {
                Err(StoreError::ReadOnly)
            } else {
                Ok(())
            };
        }
        found.get_or_insert(range);
    }
    Err(StoreError::NotReserved { found })
}

/// ESP-IDF's `readonly` flag, bit 1 - what espflash writes. Not
/// esp-bootloader-esp-idf 0.5's `is_read_only()`: that one tests bit 0,
/// which is `encrypted`.
const PART_FLAG_READONLY: u32 = 1 << 1; 
