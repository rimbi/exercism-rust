// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::u8;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let production = speed as f64 * 221.0;    
    if speed > 8 {
        return production * 0.77
    }
    if speed > 4 {
        return production * 0.9
    }
    production
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60f64) as u32
}
