//! USB CDC ACM (virtual serial port) demo for the Tomu.
//!
//! Echoes data received from the host.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg309_pac as pac;

tomu::toboot_config!();
use efm32hg309_usb::cdc_acm::{self, CdcAcmClass, CdcAcmHandler};
use efm32hg309_usb::{UsbBus, UsbDevice};

struct EchoHandler;

impl CdcAcmHandler for EchoHandler {
    fn data_received(&mut self, data: &[u8], usb: &UsbBus) {
        usb.ep_write(1, data);
    }

    fn tx_complete(&mut self, usb: &UsbBus) {
        usb.ep_prepare_out(1, 64);
    }
}

efm32hg309_usb::usb_device!(CdcAcmClass<EchoHandler>);

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

    let dev = UsbDevice::init(
        &p.cmu,
        &p.usb,
        CdcAcmClass::new(EchoHandler),
        cdc_acm::usb_config(),
    );

    defmt::info!("USB CDC ACM serial port ready");
    usb_start(dev);
    efm32hg309_usb::idle();
}
