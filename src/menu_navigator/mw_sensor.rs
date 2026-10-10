// Microwave seonsor Settings



pub const THRESHOLD_TRIGGER_MIN_VAL:f32 = 1.0;  //(18.69 - 1) default
pub const THRESHOLD_TRIGGER_MAX_VAL:f32 = 90.0;  

pub const THRESHOLD_HOLD_MIN_VAL:f32 = 1.0;  //(17.16 - 1) default
pub const THRESHOLD_HOLD_MAX_VAL:f32 = 90.0;  



pub const TT_00_INDEX: usize = 0;  // maxim 90
pub const TT_15_INDEX: usize = 15; // minim 1


pub const TH_00_INDEX: usize = 0;  // maxim 90
pub const TH_15_INDEX: usize = 15; // minim 1

/*

00=48.93
01=45.57
..
07=27.90
..
14=18.98
15=18.69


15 14 ....... 01 00
+/-
0.1


*/
