#![no_std]

use serde::Serialize;

// Re-export postcard to ensure all users use the same serde implementation+version
pub use postcard;

#[repr(C)]
#[derive(Serialize)]
pub enum IndicatorDuration {
    INFINITE,
    FINITE(u16),
}

#[repr(C)]
#[derive(Serialize)]
pub enum IndicatorState {
    OFF,
    LEFT,
    RIGHT,
    BOTH,
}

#[repr(C)]
#[derive(Serialize)]
pub enum RustIndicatorCommand {
    OFF,
    LEFT(IndicatorDuration),
    RIGHT(IndicatorDuration),
    BOTH(IndicatorDuration),
}

#[repr(C)]
pub enum BrakeLightState {
    ON,
    OFF,
}
