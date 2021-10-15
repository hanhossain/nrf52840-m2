#![no_std]
#![warn(rust_2018_idioms)]

pub mod led;
pub mod logger;

// pull in the memory layout
use nrf52840_hal as _;
