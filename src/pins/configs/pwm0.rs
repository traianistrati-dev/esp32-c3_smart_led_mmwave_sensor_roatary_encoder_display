use esp_hal::gpio::DriveMode;
use esp_hal::gpio::interconnect::PeripheralOutput;
use esp_hal::ledc::channel::{self, ChannelIFace};
use esp_hal::ledc::timer::{self, TimerIFace};
use esp_hal::ledc::{Ledc, LowSpeed};
use esp_hal::time::Rate;

// <<< GENERATED>>>
// Peripheral config (from the Virtual Module) — auto-updated; edit in the module.
pub const FREQUENCY_HZ: u32 = 5000;
// Duty resolution in BITS. Tied to FREQUENCY_HZ: 2^bits must stay
// under 80_000_000 / FREQUENCY_HZ (the LEDC runs off the 80 MHz APB
// clock), or `configure` returns Err(Divisor). Change one, check the
// other. `get_duty_resolution_bit()` below turns it into the enum.
pub const DUTY_RESOLUTION_BIT: u8 = 13;
// Which LEDC channel this pad drives, 0..=5.
pub const CHANNEL: u8 = 1;
pub const DUTY: u8 = 20;
// <<< GENERATED END >>>

/// The `Duty` variant `DUTY_RESOLUTION_BIT` names.
///
/// The constant is a plain `u8` so the Virtual Module can rewrite it
/// without touching this mapping, which is yours.
const fn get_duty_resolution_bit() -> timer::config::Duty {
    match DUTY_RESOLUTION_BIT {
        1 => timer::config::Duty::Duty1Bit,
        2 => timer::config::Duty::Duty2Bit,
        3 => timer::config::Duty::Duty3Bit,
        4 => timer::config::Duty::Duty4Bit,
        5 => timer::config::Duty::Duty5Bit,
        6 => timer::config::Duty::Duty6Bit,
        7 => timer::config::Duty::Duty7Bit,
        9 => timer::config::Duty::Duty9Bit,
        10 => timer::config::Duty::Duty10Bit,
        11 => timer::config::Duty::Duty11Bit,
        12 => timer::config::Duty::Duty12Bit,
        13 => timer::config::Duty::Duty13Bit,
        14 => timer::config::Duty::Duty14Bit,
        _ => timer::config::Duty::Duty8Bit,
    }
}

/// The `channel::Number` `CHANNEL` names.
fn get_channel() -> channel::Number {
    match CHANNEL {
        1 => channel::Number::Channel1,
        2 => channel::Number::Channel2,
        3 => channel::Number::Channel3,
        4 => channel::Number::Channel4,
        5 => channel::Number::Channel5,
        _ => channel::Number::Channel0,
    }
}

/// What `init` hands back.
pub type Handle<'d> = channel::Channel<'d, LowSpeed>;



/// The LEDC timer PWM0 runs on. One frequency for every channel on it.
///
/// `main.rs` keeps the value alive and lends it to `init` — the channels
/// hold a reference to it for as long as they exist.
pub fn timer<'d>(ledc: &Ledc<'d>) -> timer::Timer<'d, LowSpeed> {
    let mut t = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    t.configure(timer::config::Config {
            duty: get_duty_resolution_bit(),
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_hz(FREQUENCY_HZ),
    })
    .unwrap();
    t
}

/// PWM0 GPIO6 — the pads wired on the canvas, each on the channel
/// its own `CHANNEL` names, at its own `DUTY`.
pub fn init<'d>(
    ledc: &Ledc<'d>,
    timer: &'d timer::Timer<'d, LowSpeed>,
    out_pin: impl PeripheralOutput<'d>,
) -> Handle<'d> {

    let mut ch = ledc.channel(get_channel(), out_pin);
    ch
    .configure(channel::config::Config {
            timer,
            duty_pct: DUTY,
            drive_mode: DriveMode::PushPull,
    })
    .unwrap();
    ch
}

// ── Using PWM0 (blocking) ──
// In main.rs, after the init above:
//
//     use pins::configs::pwm0::DutyHandle as _;
//     use esp_hal::ledc::channel::ChannelIFace as _; // for start_duty_fade
//
//     // Hundredths of a percent — the Virtual Module's own scale.
//     _pwm0.set_duty_x100(5_000).ok(); // 50 %
//
//     // Or let the hardware fade for you — no CPU involved:
//     _pwm0.start_duty_fade(0, 100, 1000).ok();




