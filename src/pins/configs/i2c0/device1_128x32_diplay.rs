// <<< GENERATED>>>
// Device config (from the Virtual Module) — auto-updated; edit in the module.
// Device #1 on I2C0: 128x32 diplay 
// device-id: i2c0/1
// 7-bit address of the device on this bus — for YOUR code, not for `init`:
// an I2C master takes the address per transaction.
pub const DEVICE_ADDRESS: u8 = 0x3C;
// <<< GENERATED END >>>

// Everything below is editable — your changes are preserved on regeneration.
//
// `128x32 diplay ` is one of the devices sharing the i2c0 bus. The bus driver is
// built ONCE — see `i2c0` — and `main.rs` owns the handle; this file only says
// which address on it is yours. Write the device's own routines here and
// take the bus as an argument:
//
//     pub fn read_id<I: embedded_hal::i2c::I2c>(bus: &mut I) -> Option<u8> {
//         let mut rx = [0u8; 1];
//         bus.write_read(DEVICE_ADDRESS, &[0x00], &mut rx).ok()?;
//         Some(rx[0])
//     }
//
// Renaming this device, or removing one listed above it, renames this file,
// and what you wrote below the markers moves with it. Code elsewhere that
// names this module by its path has to follow the new name.
