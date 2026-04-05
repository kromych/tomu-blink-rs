//! LED pulse (breathing) demo for the Tomu.
//!
//! Uses SysTick interrupt at 100 kHz for software PWM on both LEDs.
//! Green and red LEDs breathe in opposite phase.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use core::cell::Cell;
use cortex_m_rt::exception;
use critical_section::Mutex;
use efm32hg309_pac as pac;

tomu::toboot_config!();

// PWM state shared with the SysTick ISR.
#[derive(Clone, Copy)]
struct PwmState {
    tick: u16,
    red_pwm: i16,
    green_pwm: i16,
    red_step: i8,
    green_step: i8,
    red_on: bool,
    green_on: bool,
}

static PWM: Mutex<Cell<PwmState>> = Mutex::new(Cell::new(PwmState {
    tick: 0,
    red_pwm: 0,
    green_pwm: 1000,
    red_step: 4,
    green_step: -10,
    red_on: true,
    green_on: true,
}));

#[exception]
fn SysTick() {
    critical_section::with(|cs| {
        let mut s = PWM.borrow(cs).get();
        let gpio = unsafe { &*pac::Gpio::ptr() };

        // Turn off LED once the duty cycle is reached.
        if s.red_on && s.tick >= s.red_pwm as u16 {
            s.red_on = false;
            gpio.pb_douttgl().write(|w| unsafe { w.bits(1 << 7) });
        }
        if s.green_on && s.tick >= s.green_pwm as u16 {
            s.green_on = false;
            gpio.pa_douttgl().write(|w| unsafe { w.bits(1 << 0) });
        }

        s.tick += 1;
        if s.tick >= 1000 {
            s.tick = 0;
            s.red_pwm += s.red_step as i16;
            s.green_pwm += s.green_step as i16;
            if s.red_pwm <= 0 || s.red_pwm >= 1000 {
                s.red_step = -s.red_step;
            }
            if s.green_pwm <= 0 || s.green_pwm >= 1000 {
                s.green_step = -s.green_step;
            }
            // Turn LEDs back on for the new cycle.
            gpio.pb_douttgl().write(|w| unsafe { w.bits(1 << 7) });
            gpio.pa_douttgl().write(|w| unsafe { w.bits(1 << 0) });
            s.red_on = true;
            s.green_on = true;
        }

        PWM.borrow(cs).set(s);
    });
}

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

    // Switch to USHFRCO/2 = 24 MHz for consistent SysTick timing.
    p.cmu.oscencmd().write(|w| w.ushfrcoen().set_bit());
    while p.cmu.status().read().ushfrcordy().bit_is_clear() {}
    p.cmu.usbcrctrl().modify(|_, w| w.en().set_bit());
    p.cmu.cmd().write(|w| w.hfclksel().ushfrcodiv2());
    while p.cmu.status().read().ushfrcodiv2sel().bit_is_clear() {}

    // SysTick at 100 kHz (24 MHz / 240 = 100 kHz).
    let mut syst = unsafe { cortex_m::Peripherals::steal() }.SYST;
    syst.set_reload(240 - 1);
    syst.clear_current();
    syst.enable_interrupt();
    syst.enable_counter();

    defmt::info!("LED pulse running");
    loop {
        cortex_m::asm::wfi();
    }
}
