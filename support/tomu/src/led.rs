use efm32hg309_hal as hal;
use embedded_hal::digital::OutputPin;

use hal::gpio::{
    pins::{PA0, PB7},
    Normal, OpenDrain, Output, PullUp,
};

pub struct LED<T>(T)
where
    T: ?Sized;

/// Public trait for leds.
pub trait LedTrait {
    /// Turn on the led.
    fn on(&mut self);
    /// Turn off the led.
    fn off(&mut self);
}

pub type GreenLED = LED<PA0<Output<OpenDrain<Normal, PullUp>>>>;
pub type RedLED = LED<PB7<Output<OpenDrain<Normal, PullUp>>>>;

/// LED struct stores all leds available in the Tomu board.
pub struct LEDs {
    pub green: GreenLED,
    pub red: RedLED,
}

impl LEDs {
    pub fn new(
        green: PA0<Output<OpenDrain<Normal, PullUp>>>,
        red: PB7<Output<OpenDrain<Normal, PullUp>>>,
    ) -> Self {
        LEDs {
            green: LED(green),
            red: LED(red),
        }
    }
}

impl<T: OutputPin> LedTrait for LED<T> {
    fn on(&mut self) {
        let _ = self.0.set_low();
    }

    fn off(&mut self) {
        let _ = self.0.set_high();
    }
}
