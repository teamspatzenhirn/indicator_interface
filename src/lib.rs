#![no_std]

use serde::{Deserialize, Serialize};

// Re-export postcard to ensure all users use the same serde implementation+version
pub use postcard;

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum IndicatorDuration {
    INFINITE,
    FINITE(u16),
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RustIndicatorCommand {
    OFF,
    LEFT(IndicatorDuration),
    RIGHT(IndicatorDuration),
    BOTH(IndicatorDuration),
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub enum BrakeLightState {
    ON,
    OFF,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum I2CMessage {
    BrakeLight(BrakeLightState),
    Indicator(RustIndicatorCommand),
}
