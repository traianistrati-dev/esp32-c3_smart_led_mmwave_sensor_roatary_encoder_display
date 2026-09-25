

use crate::pins::utils::pwm_fade::duty_from_percent;

pub const MAX_DUTY: u32 = duty_from_percent(95.0);


//--------------------------------

pub struct Settings{
    pub working: DutyAndDuration
    , normal: DutyAndDuration
    , night: DutyAndDuration
}


// enum SettingModeAction{
// Normal(DutyAndDuration)
// , Night(DutyAndDuration)

// }
//-----------------------------------------------------
pub struct OptionCloseRangeBoost{
    pub duty: u32 //Max Light %
    , sec: u8 //Activate after se
    , detection_distance_cm: u16 //Activate under cm
}

impl OptionCloseRangeBoost{

    pub fn new(duty: u32, sec: u8, detection_distance_cm: u16) -> Self{

        Self{
            duty, sec, detection_distance_cm
        }
    }

    pub const fn default() -> Self {
        Self { duty: 50, sec: 5, detection_distance_cm: 150 }
    }
}


//-----------------------------------------------------




impl Settings{

    pub const fn new() ->Self{

        Self{
            // working: DutyAndDuration::new(pins::utils::pwm_fade::duty_from_percent(10.0), 3000, 5000),
            // working: DutyAndDuration::new(pins::utils::pwm_fade::duty_from_percent(10.0), 1000, 2000),

            // working: DutyAndDuration::new(pins::utils::pwm_fade::duty_from_percent(50.0), 3000, 5000),
            // working: DutyAndDuration::new(pins::utils::pwm_fade::duty_from_percent(50.0), 1000, 2000),

            working: DutyAndDuration::new(MAX_DUTY, 1000, 2000, 3),
            //working: DutyAndDuration::new(MAX_DUTY, 3000, 5000),
            //------------------------------------------

            normal: DutyAndDuration::new(duty_from_percent(50.0), 3000, 5000, 10),

            night: DutyAndDuration::new(duty_from_percent(10.0), 3000, 5000, 5),

        }
    }

    pub fn get_fade_in_increment_time(&self) -> u32{
        let mut step: f32 = self.working.fade_in_milis as f32 / self.working.duty as f32;
        if step == 0.0 {step = 0.1;}
        (step * 1000.0) as u32
    }

    pub fn get_fade_out_increment_time(&self) -> u32{
        let mut step: f32 = self.working.fade_out_milis as f32 / self.working.duty as f32;
        if step == 0.0 {step = 0.1;}
        (step * 1000.0) as u32
    }

    pub fn get_on_time_micros(&self) -> u32{
        (self.working.on_time_sec as u32 * 1_000_000) as u32
    }


}



pub struct DutyAndDuration{
    pub duty: u32
    , fade_in_milis: u32
    , fade_out_milis: u32
    , on_time_sec: u8
    , close_range_boost: Option<OptionCloseRangeBoost>
}



impl DutyAndDuration{
    const fn new(duty: u32, fade_in_milis: u32, fade_out_milis: u32, on_time_sec: u8) -> Self{
        Self{duty, fade_in_milis, fade_out_milis, on_time_sec, close_range_boost: None}
    }

    const fn new_with_close_range_boost(duty: u32, fade_in_milis: u32, fade_out_milis: u32, on_time_sec: u8) -> Self{
        Self{duty, fade_in_milis, fade_out_milis, on_time_sec, close_range_boost: Some(OptionCloseRangeBoost::default())}
    }


    pub fn set_on_time_sec(&mut self, sec: u8){
        self.on_time_sec = sec;
    }

    pub fn set_fade_in_sec(&mut self, sec: u8){
        self.fade_in_milis = sec as u32 * 1000;
    }

    pub fn set_fade_out_sec(&mut self, sec: u8){
        self.fade_out_milis = sec as u32 * 1000;
    }

    pub fn set_max_duty_percents(&mut self, percent: f32){

        self.duty = duty_from_percent(percent);
    }
}