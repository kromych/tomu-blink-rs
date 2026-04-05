//! USB MIDI device demo for the Tomu.
//!
//! Plays "Twinkle Twinkle Little Star" in a loop once configured.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg309_pac as pac;

tomu::toboot_config!();
use efm32hg309_usb::midi::{self, MidiClass, MidiHandler};
use efm32hg309_usb::UsbDevice;

struct TwinkleTune;

impl MidiHandler for TwinkleTune {
    fn tune(&self) -> &[(u8, u32)] {
        &TUNE
    }
}

// "Twinkle Twinkle Little Star" - (MIDI note, duration in ms)
#[rustfmt::skip]
static TUNE: [(u8, u32); 42] = [
    (60, 400), (60, 400), (67, 400), (67, 400),
    (69, 400), (69, 400), (67, 800),
    (65, 400), (65, 400), (64, 400), (64, 400),
    (62, 400), (62, 400), (60, 800),
    (67, 400), (67, 400), (65, 400), (65, 400),
    (64, 400), (64, 400), (62, 800),
    (67, 400), (67, 400), (65, 400), (65, 400),
    (64, 400), (64, 400), (62, 800),
    (60, 400), (60, 400), (67, 400), (67, 400),
    (69, 400), (69, 400), (67, 800),
    (65, 400), (65, 400), (64, 400), (64, 400),
    (62, 400), (62, 400), (60, 800),
];

efm32hg309_usb::usb_device!(MidiClass);

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

    let dev = UsbDevice::init(&p.cmu, &p.usb, MidiClass, midi::usb_config());

    defmt::info!("USB MIDI device ready");
    usb_start(dev);
    midi::run(&TwinkleTune, |pkt| usb_with_bus(|bus| bus.ep_write(1, pkt)));
}
