#![no_std]
#![no_main]

// pull in the memory layout
use nrf52840_m2_dk as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use log::LevelFilter;
use nrf52840_m2::logger;

#[entry]
fn main() -> ! {
    logger::init_with_level(LevelFilter::Trace);
    log::info!("Hello world from nrf52840-m2-dk!");

    loop {
        asm::bkpt();
    }
}
