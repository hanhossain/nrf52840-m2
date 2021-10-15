#![no_std]
#![no_main]

mod leds;
mod logger;

// pull in the memory layout
use nrf52840_hal as _;

use crate::leds::Led;
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
    let mut red = Led::red(pins.p0_30);
    let mut green = Led::green(pins.p0_29);
    let mut blue = Led::blue(pins.p0_31);

    log::debug!("Configured LEDs");

    let core_peripherals = CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core_peripherals.SYST);

    // blink led1 (red), p0.30
    red.on();
    sleep(&mut delay);

    red.off();
    sleep(&mut delay);

    // blink led2 (green), p0.29
    green.on();
    sleep(&mut delay);

    green.off();
    sleep(&mut delay);

    // blink led3 (blue), p0.31
    blue.on();
    sleep(&mut delay);

    blue.off();
    sleep(&mut delay);

    loop {
        asm::bkpt();
    }
}

fn sleep(delay: &mut Delay) {
    delay.delay_ms(1000u16);
}
