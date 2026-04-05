//! USB HID keyboard demo for the Tomu.
//!
//! Types "Hello from Tomu!\n" every 5 seconds.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg309_pac as pac;

tomu::toboot_config!();
use efm32hg309_usb::hid_keyboard::{self, HidKeyboardClass, HidKeyboardHandler};
use efm32hg309_usb::UsbDevice;

struct Greeting;

impl HidKeyboardHandler for Greeting {
    fn message(&self) -> &[u8] {
        b"Hello from Tomu!\n"
    }
}

efm32hg309_usb::usb_device!(HidKeyboardClass);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    p.wdog.ctrl().write(|w| w.en().clear_bit());

    // Enable GPIO clock; read-back ensures the clock is active before GPIO access.
    p.cmu.hfperclken0().modify(|_, w| w.gpio().set_bit());
    let _ = p.cmu.hfperclken0().read();

    // Set DOUT high (LEDs off) before configuring pin mode so LEDs never glitch on.
    p.gpio
        .pa_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 0)) });
    p.gpio
        .pb_dout()
        .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 7)) });
    p.gpio.pa_model().modify(|_, w| w.mode0().wiredand());
    p.gpio.pb_model().modify(|_, w| w.mode7().wiredand());

    let dev = UsbDevice::init(&p.cmu, &p.usb, HidKeyboardClass, hid_keyboard::usb_config());

    defmt::info!("USB HID keyboard ready");
    usb_start(dev);
    hid_keyboard::run(&Greeting, |report| {
        usb_with_bus(|bus| bus.ep_write(1, report))
    });
}
