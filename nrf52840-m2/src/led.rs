use nrf52840_hal::gpio::p0::{P0_29, P0_30, P0_31};
use nrf52840_hal::gpio::{Level, Output, Pin, PushPull};
use nrf52840_hal::prelude::*;

/// LED wrapper around a pin
pub struct Led {
    pin: Pin<Output<PushPull>>,
}

impl Led {
    /// Wraps the pin in the LED wrapper
    fn new(pin: Pin<Output<PushPull>>) -> Led {
        Led { pin }
    }

    /// Creates the red LED (LED1, p0.30)
    pub fn red<T>(pin: P0_30<T>) -> Led {
        let pin = pin.degrade().into_push_pull_output(Level::High);
        Led::new(pin)
    }

    /// Creates the green LED (LED2, p0.29)
    pub fn green<T>(pin: P0_29<T>) -> Led {
        let pin = pin.degrade().into_push_pull_output(Level::High);
        Led::new(pin)
    }

    /// Creates the blue LED (LED3, p0.31)
    pub fn blue<T>(pin: P0_31<T>) -> Led {
        let pin = pin.degrade().into_push_pull_output(Level::High);
        Led::new(pin)
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pin.set_low().unwrap();
    }

    /// Turns the LED off
    pub fn off(&mut self) {
        self.pin.set_high().unwrap();
    }
}
