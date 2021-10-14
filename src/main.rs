#![no_std]
#![no_main]

mod logger;

use nrf52840_hal as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use log::LevelFilter;

#[entry]
fn main() -> ! {
    logger::init_with_level(LevelFilter::Trace);
    log::info!("Hello world!");

    loop {
        asm::bkpt();
    }
}
