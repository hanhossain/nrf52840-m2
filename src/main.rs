#![no_std]
#![no_main]

mod logger;

// pull in the memory layout
use nrf52840_hal as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use log::LevelFilter;
use nrf52840_hal::gpio::{p0, Level};
use nrf52840_hal::pac::{CorePeripherals, Peripherals};
use nrf52840_hal::prelude::*;
use nrf52840_hal::Delay;

#[entry]
fn main() -> ! {
    logger::init_with_level(LevelFilter::Trace);
    log::info!("Hello world!");

    let peripherals = Peripherals::take().unwrap();

    let pins = p0::Parts::new(peripherals.P0);
    let mut led = pins.p0_30.degrade().into_push_pull_output(Level::High);

    log::debug!("Pin P0-30 configured for output (red led)");

    let core_peripherals = CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core_peripherals.SYST);

    loop {
        // blink led1 (red), p0.30
        log::debug!("Led on");
        led.set_low().unwrap();
        delay.delay_ms(1000u16);

        log::debug!("Led off");
        led.set_high().unwrap();
        delay.delay_ms(1000u16);
    }
}
