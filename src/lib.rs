#![no_std]
#![no_main]

pub mod led;
pub mod logger;

// pull in the memory layout
use nrf52840_hal as _;
