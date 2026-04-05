#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg309_pac as pac;

tomu::toboot_config!();

#[cortex_m_rt::entry]
fn main() -> ! {
    tomu::toboot::verify_config();

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

    loop {
        // Green on, red off
        p.gpio.pa_doutclr().write(|w| unsafe { w.bits(1 << 0) });
        p.gpio.pb_doutset().write(|w| unsafe { w.bits(1 << 7) });
        cortex_m::asm::delay(14_000 * 500); // ~500ms at 14 MHz

        // Red on, green off
        p.gpio.pa_doutset().write(|w| unsafe { w.bits(1 << 0) });
        p.gpio.pb_doutclr().write(|w| unsafe { w.bits(1 << 7) });
        cortex_m::asm::delay(14_000 * 500);
    }
}
