//! Meniu de setări complet static.
//! - arborele de noduri stă în flash (.rodata), 0 bytes RAM
//! - în RAM ajung doar valorile (AtomicU32) și MenuNav
//! - fără lifetimes, fără inițializare la runtime

use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};
use serde::{Deserialize, Serialize};


// ------------------------------------------------------------------
// Meniuri
// ------------------------------------------------------------------

pub const BACK: Node = Node { name: "<<", kind: Kind::Back};
pub const ENABLED_NAME: &str = "Enabled";
pub const DISABLED_NAME: &str = "Disabled";

macro_rules! mode_menu {
    ($menu: ident, $crb: ident, $v: ident) => {


        static $crb: [Node; 5] = [
            Node::value_enabled(ENABLED_NAME, &$v.enabled_boost),
            Node::value("Max Light %", &$v.crb_max_light),
            Node::value("Activate after sec", &$v.crb_after_sec),
            Node::value("Activate under cm", &$v.crb_under_cm),
            BACK,
        ];


        static $menu: [Node; 7] = [
            Node::value_enabled(ENABLED_NAME, &$v.enabled),
            Node::value("Fade IN sec", &$v.fade_in),
            Node::value("Absence ON sec", &$v.duration_on),
            Node::value("Fade OUT sec", &$v.fade_out),
            Node::value("Max Light %", &$v.max_light),
            Node::submenu("Close-Range Boost", &$crb),
            BACK,
        ];

    };
}

mode_menu!(NORMAL_MENU, NORMAL_CRB_MENU, NORMAL);
mode_menu!(NIGHT_MENU, NIGHT_CRB_MENU, NIGHT);



static SENSOR_MENU: [Node; 3] = [
    Node::value("Detection Range", &SENSOR.range),
    Node::value("Detection sensivity", &SENSOR.sensivity),
    BACK,
];

static LIGHT_MENU: [Node; 2] = [
    Node::value("Max Light %", &LIGHT.max_light_p),
    BACK,
];

static SYSTEM_MENU: [Node; 2] = [
    Node::value_time("Curent Time HH:mm", &SYSTEM.curent_time),
    BACK,
];



pub static ROOT: [Node; 5] = [

    Node::submenu("Working Modes", &[
            Node::submenu("Normal", &NORMAL_MENU),
            Node::submenu("Night", &NIGHT_MENU),
            BACK,
    ]),
    Node::submenu("Sensor", &SENSOR_MENU),
    Node::submenu("Light", &LIGHT_MENU),
    Node::submenu("System", &SYSTEM_MENU),
    BACK, // <<
];



// ------------------------------------------------------------------
// Valori
// ------------------------------------------------------------------

use crate::menu_navigator::menu_types::{bool_value::ValueBool, number_value::Value, time_value::ValueTime};


#[derive(Serialize, Deserialize, PartialEq)]
pub struct ModeValues {
    pub fade_in: Value,
    pub duration_on: Value,
    pub fade_out: Value,
    pub max_light: Value,
    pub crb_max_light: Value,
    pub crb_after_sec: Value,
    pub crb_under_cm: Value,
    pub enabled: ValueBool,
    pub enabled_boost: ValueBool,
}


impl ModeValues {
    pub fn clone(&self) -> Self{
        Self{
            fade_in: Value::new(self.fade_in.min, self.fade_in.max, self.fade_in.get(), self.fade_in.step, self.fade_in.as_float)
            , duration_on: Value::new(self.duration_on.min, self.duration_on.max, self.duration_on.get(), self.duration_on.step, self.duration_on.as_float)
            , fade_out: Value::new(self.fade_out.min, self.fade_out.max, self.fade_out.get(), self.fade_out.step, self.fade_out.as_float)
            , max_light: Value::new(self.max_light.min, self.max_light.max, self.max_light.get(), self.max_light.step, self.max_light.as_float)
            , crb_max_light: Value::new(self.crb_max_light.min, self.crb_max_light.max, self.crb_max_light.get(), self.crb_max_light.step, self.crb_max_light.as_float)
            , crb_after_sec: Value::new(self.crb_after_sec.min, self.crb_after_sec.max, self.crb_after_sec.get(), self.crb_after_sec.step, self.crb_after_sec.as_float)
            , crb_under_cm: Value::new(self.crb_under_cm.min, self.crb_under_cm.max, self.crb_under_cm.get(), self.crb_under_cm.step, self.crb_under_cm.as_float)

            , enabled: ValueBool::new_from(self.enabled.get())
            , enabled_boost: ValueBool::new_from(self.enabled_boost.get())

        }

    }

    pub fn load(&self, values: ModeValues){
        self.fade_in.set(values.fade_in.get());
        self.duration_on.set(values.duration_on.get());
        self.fade_out.set(values.fade_out.get());
        self.max_light.set(values.max_light.get());
        self.crb_max_light.set(values.crb_max_light.get());
        self.crb_after_sec.set(values.crb_after_sec.get());
        self.crb_under_cm.set(values.crb_under_cm.get());
        self.enabled.set(values.enabled.get());
        self.enabled_boost.set(values.enabled_boost.get())
    }

}
//-----------------------------------------------------------

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SensorValues {
    pub sensivity: Value,
    pub range: Value,
}

impl SensorValues {
    pub fn clone(&self) -> Self{
        Self{
            sensivity: Value::new(self.sensivity.min, self.sensivity.max, self.sensivity.get(), self.sensivity.step, self.sensivity.as_float)
            , range: Value::new(self.range.min, self.range.max, self.range.get(), self.range.step, self.range.as_float)
        }
    }
}

//-----------------------------------------------------------

#[derive(Serialize, Deserialize, PartialEq)]
pub struct LightValues {
    pub max_light_p: Value,
}

impl LightValues {
    pub fn clone(&self) -> Self{
        Self{
            max_light_p: Value::new(self.max_light_p.min, self.max_light_p.max, self.max_light_p.get() , self.max_light_p.step, self.max_light_p.as_float)
        }
    }
}

//-----------------------------------------------------------

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SystemValues {
    pub curent_time: ValueTime,
}

impl SystemValues {
    pub fn clone(&self) -> Self{
        Self{
            curent_time: ValueTime::new_from(self.curent_time.get().0 , self.curent_time.get().1)
        }
    }
}
//-----------------------------------------------------------


pub static NORMAL: ModeValues = ModeValues {
    fade_in: Value::new(1, 10, 0, 1, false),
    duration_on: Value::new(0, 180, 0, 1, false),
    fade_out: Value::new(1, 30, 0, 1, false),
    max_light: Value::new(10, 90, 0, 1, false),

    crb_max_light: Value::new(50, 100, 0, 1, false),
    crb_after_sec: Value::new(3, 20, 0, 1, false),
    crb_under_cm: Value::new(40, 500, 0, 10, true),
    enabled: ValueBool::new(),
    enabled_boost: ValueBool::new(),
};

pub static NIGHT: ModeValues = ModeValues {
    fade_in: Value::new(1, 10, 0, 1, false),
    duration_on: Value::new(0, 120, 0, 1, false),
    fade_out: Value::new(1, 10, 0, 1, false),
    max_light: Value::new(1, 20, 0, 1, false),

    crb_max_light: Value::new(20, 100, 0, 1, false),
    crb_after_sec: Value::new(3, 20, 0, 1, false),
    crb_under_cm: Value::new(40, 500, 0, 10, true),
    enabled: ValueBool::new(),
    enabled_boost: ValueBool::new(),

};


pub static SENSOR: SensorValues = SensorValues {
    sensivity: Value::new(20, 90, 0, 1, false),
    range: Value::new(0, 15, 0, 1, false),

};


pub static LIGHT: LightValues = LightValues {
    max_light_p: Value::new(1, 100, 100, 1, false),

};

pub static SYSTEM: SystemValues = SystemValues {
    curent_time: ValueTime::new(),

};



// ------------------------------------------------------------------
// Noduri
// ------------------------------------------------------------------

#[derive(Clone, Copy)]
pub enum Kind {
    Back,
    Submenu(&'static [Node]),
    Value(&'static Value),
    ValueTime(&'static ValueTime),
    ValueBool(&'static ValueBool),
}

pub struct Node {
    pub name: &'static str,
    pub kind: Kind,
}


impl Node {

    pub const fn submenu(name: &'static str, items: &'static [Node]) -> Self {
        Self { name, kind: Kind::Submenu(items)}
    }

    pub const fn value(name: &'static str, v: &'static Value) -> Self {
        Self { name, kind: Kind::Value(v)}
    }


    pub const fn value_time(name: &'static str, v: &'static ValueTime) -> Self {
        Self { name, kind: Kind::ValueTime(v)}
    }


    pub const fn value_enabled(name: &'static str, v: &'static ValueBool) -> Self {
        Self { name, kind: Kind::ValueBool(v)}
    }

    pub fn get_name(&self) -> &'static str{


        if let Kind::ValueBool(b) = self.kind{
            return if b.get() {ENABLED_NAME} else {DISABLED_NAME}
        }
        self.name
    }

    pub fn get_name_enable(&self) -> &'static str{


        if let Kind::ValueBool(b) = self.kind{
            return if b.get() {"Disable"} else {"Enable"}
        }
        self.name
    }

}





// ------------------------------------------------------------------
// Navigare
// ------------------------------------------------------------------

const DEPTH: usize = 4;

// #[derive(Debug)]
pub enum Action {
    None,
    ExitSettings,
    EditValue(&'static Value),
    EditValueTime(&'static ValueTime),
    Enabled(&'static ValueBool),
	Off
}

pub struct MenuNav {

    stack: [(&'static [Node], usize); DEPTH], // (meniu, cursor)
    pub depth: usize,
    init: bool,
}

impl MenuNav {
    pub fn new() -> Self {
        let root: &'static [Node] = &ROOT;
        Self { stack: [(root, 0); DEPTH], depth: 0, init: false}
    }

    /// Readuce meniul la ROOT (ex. la fiecare intrare în setări).
    pub fn reset(&mut self) {
        *self = Self::new();
    }


    pub fn items(&self) -> impl Iterator<Item = &'static Node> + Clone {
        let disabled = self.contains_node_disabled();

        self.stack[self.depth].0
        .iter()
        .filter(move |e| {
                if disabled{
                    return e.get_name() == DISABLED_NAME || e.name == BACK.name;
                }
                true

        })
    }

    pub fn cursor(&self) -> usize {
        self.stack[self.depth].1
    }


    pub fn next(&mut self) {
        let items_count = self.items().count();
        let (_, c) = &mut self.stack[self.depth];

        *c = (*c + 1) % items_count;
    }

    pub fn prev(&mut self) {
        let items_count = self.items().count();

        let (_, c) = &mut self.stack[self.depth];

        *c = (*c + items_count - 1) % items_count;
    }

    pub fn select(&mut self) -> Action {
        let (menu, c) = self.stack[self.depth];
        match menu[c].kind {
            Kind::Back if self.depth == 0 => Action::ExitSettings,
            Kind::Back => {
                self.depth -= 1; // cursorul părintelui rămâne salvat
                Action::None
            },
            Kind::Submenu(sub) => {
                if self.init && self.depth + 1 < DEPTH {
                    self.depth += 1;
                    self.stack[self.depth] = (sub, 0);
                } else if !self.init{
                    self.init = true;
                }


                Action::None
            },

            Kind::Value(v) => {
                //go back if disabled
                if menu[0].get_name() == DISABLED_NAME && c == 1{
                    self.depth -= 1;
                    return Action::None;
                }
                Action::EditValue(v)

            },
            Kind::ValueBool(value_enabled) => Action::Enabled(value_enabled),
            Kind::ValueTime(value_time) => Action::EditValueTime(value_time),
        }
    }


    pub fn selected_node(& self) -> &'static Node{
        let (menu, c) = self.stack[self.depth];
        &menu[c]
    }

    pub fn contains_node_disabled(&self) -> bool{
        let (menu, _) = self.stack[self.depth];
        menu[0].get_name() == DISABLED_NAME
    }


}

// ------------------------------------------------------------------
// Usecase Example
// ------------------------------------------------------------------
//
// let mut nav = MenuNav::new();
//
// // print o ndisplay
// for (i, node) in nav.items().iter().enumerate() {
//     let marker = if i == nav.cursor() { ">" } else { " " };
//     match node.kind {
//         Kind::Value(v) => { /* "{marker}{node.name}: {v.get()}" */ }
//         _              => { /* "{marker}{node.name}" */ }
//     }
// }
//
// // button OK
// match nav.select() {
//     Action::ExitSettings => { /* salvezi în flash, ieși din setări */ }
//     Action::EditValue(v) => { /* mod editare: v.inc(1) / v.dec(1) */ }
//     Action::None => {}
// }