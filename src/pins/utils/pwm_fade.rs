


// FADE
use esp_hal::ledc::channel::ChannelHW;

use crate::pins::configs::pwm0::{Handle, DUTY_RESOLUTION_BIT};





/// Highest raw duty the timer accepts: 2^bits - 1.
pub const DUTY_RAW_MAX: u32 = (1u32 << DUTY_RESOLUTION_BIT) - 1;

#[inline]
pub const fn duty_from_percent(percent: f32) -> u32 {
    (DUTY_RAW_MAX as f32 * (percent / 100.0)) as u32

}


/// Owns the channel and remembers where the light currently is, so a fade
/// starts from the real value instead of assuming 0 or full.
pub struct Dimmer<'d> {
    ch: Handle<'d>,
}

impl<'d> Dimmer<'d> {

    pub fn new(ch: Handle<'d>) -> Self {
        Self { ch}
    }

    #[inline]
    pub fn set_duty_hw_(&mut self, duty: u32) {
        if duty >= 0{
            self.ch.set_duty_hw(duty);
        }
    }

}