use esp_hal::{Async, Blocking};
use esp_hal::gpio::interconnect::{PeripheralInput, PeripheralOutput};
use esp_hal::i2c::master::{Config, I2c, Instance};
use esp_hal::time::Rate;

// <<< GENERATED>>>
// Peripheral config (from the Virtual Module) — auto-updated; edit in the module.
pub const FREQUENCY_HZ: u32 = 400000;
// 7-bit address of the device on this bus — for YOUR code, not for `init`:
// an esp-hal I2C master takes the address per transaction.
pub const DEVICE_ADDRESS: u8 = 0x00;
// <<< GENERATED END >>>

/// I2C0 master — blocking driver.
pub fn init<'d>(
    i2c: impl Instance + 'd,
    scl: impl PeripheralInput<'d> + PeripheralOutput<'d>,
    sda: impl PeripheralInput<'d> + PeripheralOutput<'d>,
) -> I2c<'d, Blocking> {
    let config = Config::default().with_frequency(Rate::from_hz(FREQUENCY_HZ));
    I2c::new(i2c, config)
        .unwrap()
        .with_scl(scl)
        .with_sda(sda)
}

/// I2C0 master — async driver.
///
/// Same construction as `init`, then `.into_async()` — the methods become
/// `*_async` and `.await`-able on the embassy executor.
pub fn init_async<'d>(
    i2c: impl Instance + 'd,
    scl: impl PeripheralInput<'d> + PeripheralOutput<'d>,
    sda: impl PeripheralInput<'d> + PeripheralOutput<'d>,
) -> I2c<'d, Async> {
    let config = Config::default().with_frequency(Rate::from_hz(FREQUENCY_HZ));
    I2c::new(i2c, config)
        .unwrap()
        .with_scl(scl)
        .with_sda(sda)
        .into_async()
}

// ── Using I2C0 (async) ──
// main.rs calls `init_async`, so the handle is an `I2c<'_, Async>`:
//
//     use pins::configs::i2c0::DEVICE_ADDRESS;
//
//     _i2c0.write_async(DEVICE_ADDRESS, &[0x10, 0x42]).await.ok();
//
//     let mut rx = [0u8; 2];
//     _i2c0.read_async(DEVICE_ADDRESS, &mut rx).await.ok();
//
//     // Register read (repeated START) — what most sensors expect.
//     _i2c0.write_read_async(DEVICE_ADDRESS, &[0x10], &mut rx).await.ok();
//
//     // `init` is still there if you want this bus blocking instead.
