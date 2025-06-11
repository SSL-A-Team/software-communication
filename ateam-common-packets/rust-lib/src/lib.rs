#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::bindings::KickerTelemetry;

pub mod bindings;

pub mod radio;

impl Default for KickerTelemetry {
    fn default() -> Self {
        Self { 
            _bitfield_align_1: Default::default(),
            _bitfield_1: Default::default(),
            charge_pct: Default::default(),
            rail_voltage: Default::default(),
            battery_voltage: Default::default(),
            dribbler: Default::default() 
        }
    }
}
