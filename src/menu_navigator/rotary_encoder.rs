use embedded_hal::digital::InputPin;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncoderEvent {
    None,
    Left,
    Right,
    Pressed,
}

impl EncoderEvent{

    #[allow(dead_code)]
    #[inline]
    pub const fn is_pressed(self) -> bool {
        matches!(self, EncoderEvent::Pressed)
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn is_left(self) -> bool {
        matches!(self, EncoderEvent::Left)
    }

    #[allow(dead_code)]
    #[inline]
    pub const fn is_right(self) -> bool {
        matches!(self, EncoderEvent::Right)
    }

    /// -1 / 0 / +1 — usefull for direct index increment
    #[allow(dead_code)]
    #[inline]
    pub const fn delta(self) -> i32 {
        match self {
            EncoderEvent::Right => 1,
            EncoderEvent::Left => -1,
            _ => 0,
        }
    }

}



const TRANSITIONS: [i8; 16] = [
    0, -1, 1, 0,
    1, 0, 0, -1,
    -1, 0, 0, 1,
    0, 1, -1, 0,
];

pub struct RotaryEncoder<CLK, DT, SW> {
    clk: CLK,
    dt: DT,
    sw: SW,
    state: u8, // ultimii 2 biți: (clk << 1) | dt
    accum: i8, // accumulated sub-steps
    last_sw: bool,
    // pub value: i32,
    step: i8
}

const DEFAULT_STEP: i8 = 2;

impl<CLK: InputPin, DT: InputPin, SW: InputPin> RotaryEncoder<CLK, DT, SW> {

    pub fn new(mut clk: CLK, mut dt: DT, mut sw: SW) -> Self {
        let state = ((clk.is_high().unwrap_or(false) as u8) << 1) | (dt.is_high().unwrap_or(false) as u8);
        let last_sw = sw.is_high().unwrap_or(false);

        Self { clk, dt, sw, state, accum: 0, last_sw, step: DEFAULT_STEP}
        // Self { clk, dt, sw, state, accum: 0, last_sw, value: 0 }
    }

    pub fn set_step(&mut self, step: u8){

        self.step = if step < 5 {
            (step * 2) as i8
        } else{ DEFAULT_STEP }

    }

    pub fn update(&mut self) -> EncoderEvent {
        let sw = self.sw.is_high().unwrap_or(false);
        let pressed = self.last_sw && !sw; // front descendent = apăsat
        self.last_sw = sw;

        if pressed { return EncoderEvent::Pressed;}

        let clk = self.clk.is_high().unwrap_or(false) as u8;
        let dt = self.dt.is_high().unwrap_or(false) as u8;
        let now = (clk << 1) | dt;

        if now != self.state {
            let idx = ((self.state << 2) | now) & 0x0F;
            self.accum += TRANSITIONS[idx as usize];
            self.state = now;


            // 4 == 2 sub-steps for 1 „clic"
            // 2 == 1 sub-steps for 1 „clic"
            if self.accum >= self.step {
                self.accum = 0;
                // self.value += 1;
                return EncoderEvent::Right;
            }
            else if self.accum <= -self.step {
                self.accum = 0;
                // self.value -= 1;
                return EncoderEvent::Left;
            }
        }

        EncoderEvent::None
    }
}
use embassy_futures::select::select3;
use embedded_hal_async::digital::Wait;

impl<CLK, DT, SW> RotaryEncoder<CLK, DT, SW>
where
CLK: InputPin + Wait,
DT: InputPin + Wait,
SW: InputPin + Wait,
{
    pub async fn wait_event(&mut self) -> EncoderEvent {
        loop {
            let ev = self.update();
            if ev != EncoderEvent::None {
                return ev;
            }
            // doarme până se schimbă oricare pin
            let _ = select3(
                self.clk.wait_for_any_edge(),
                self.dt.wait_for_any_edge(),
                self.sw.wait_for_any_edge(),
            )
            .await;
        }
    }
}



