

use embedded_hal::digital::{InputPin};
use crate::menu_navigator::rotary_encoder::{RotaryEncoder};

// <<< GENERATED>>>
// Custom module "menu nav" — one field per pin, in the order they
// were added in the Virtual-module panel. Generic over each pin's
// type, so it compiles whatever the pins bind to.
//#[derive(Debug)]
pub struct Encoder<GPIO1, GPIO2, GPIO3> {

    pub encoder: RotaryEncoder<GPIO1, GPIO2, GPIO3>
}

// <<< GENERATED END >>>


// Your own methods for this module. The bounds come from each pin's
// direction, so `.is_high()` / `.set_high()` are available here.
// NB: if you add or remove pins and press Update, adjust this generic
// list to match the regenerated struct above.
impl<GPIO1: InputPin, GPIO2: InputPin, GPIO3: InputPin> Encoder<GPIO1, GPIO2, GPIO3> {

    pub fn new(gpio1_in_clk: GPIO1, gpio2_in_dt: GPIO2, gpio3_in_sw: GPIO3) -> Self{
        Self{
            encoder: RotaryEncoder::new(gpio1_in_clk, gpio2_in_dt, gpio3_in_sw)
        }
    }

}
