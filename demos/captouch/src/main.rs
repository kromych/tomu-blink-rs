//! Capacitive touch "coin flip" demo for the Tomu.
//!
//! Touch the two outer pads on the Tomu to flip a coin.
//! Green = heads, red = tails. Result determined by internal tick parity.
//!
//! Uses ACMP0 in capsense mode, PRS to route ACMP output to TIMER1,
//! and TIMER0 as a measurement window generator.

#![no_main]
#![no_std]

use cortex_m as _;
use defmt_rtt as _;
use panic_halt as _;

use efm32hg309_pac as pac;

tomu::toboot_config!();
use pac::interrupt;
use portable_atomic::{AtomicBool, AtomicU32, Ordering};

// Capsense state shared with TIMER0 ISR.
static CAPSENSE_GEN: AtomicU32 = AtomicU32::new(0);
static CHANNEL_VALUES: [AtomicU32; 4] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];
static CURRENT_CHANNEL: AtomicU32 = AtomicU32::new(0);
static CAPSENSE_RUNNING: AtomicBool = AtomicBool::new(false);

const CAPSENSE_DETECT_MIN: u32 = 20;
const LED_ON_TICKS: u32 = 1000;
const LED_OFF_TICKS: u32 = 2000;

/// Set ACMP capsense channel.
fn acmp_set_channel(channel: u32) {
    let acmp = unsafe { &*pac::Acmp0::ptr() };
    // INPUTSEL: CSRESEN | NEGSEL=CAPSENSE(7) | POSSEL=channel | VDDLEVEL
    let vddlevel: u32 = if channel == 0 { 0x3F } else { 0x3D };
    acmp.inputsel().write(|w| unsafe {
        w.bits(
            (1 << 24)             // CSRESEN
            | (vddlevel << 8)     // VDDLEVEL
            | (7 << 4)            // NEGSEL = CAPSENSE
            | channel, // POSSEL = channel
        )
    });
}

/// Start a capsense measurement on the given channel.
fn capsense_measure(channel: u32) {
    CURRENT_CHANNEL.store(channel, Ordering::Relaxed);
    acmp_set_channel(channel);

    let timer0 = unsafe { &*pac::Timer0::ptr() };
    let timer1 = unsafe { &*pac::Timer1::ptr() };

    // Reset and start both timers.
    timer0.cnt().write(|w| unsafe { w.bits(0) });
    timer1.cnt().write(|w| unsafe { w.bits(0) });
    timer0.cmd().write(|w| w.start().set_bit());
    timer1.cmd().write(|w| w.start().set_bit());
}

#[interrupt]
fn TIMER0() {
    let timer0 = unsafe { &*pac::Timer0::ptr() };
    let timer1 = unsafe { &*pac::Timer1::ptr() };

    // Stop both timers.
    timer0.cmd().write(|w| w.stop().set_bit());
    timer1.cmd().write(|w| w.stop().set_bit());

    // Clear overflow interrupt.
    timer0.ifc().write(|w| w.of().set_bit());

    // Read TIMER1 count = number of ACMP oscillations.
    let count = timer1.cnt().read().bits();
    let ch = CURRENT_CHANNEL.load(Ordering::Relaxed);
    if (ch as usize) < 4 {
        CHANNEL_VALUES[ch as usize].store(count, Ordering::Relaxed);
    }

    if CAPSENSE_RUNNING.load(Ordering::Relaxed) {
        let next = if ch >= 1 {
            CAPSENSE_GEN.fetch_add(1, Ordering::Relaxed);
            0
        } else {
            ch + 1
        };
        capsense_measure(next);
    }
}

fn setup_capsense() {
    let cmu = unsafe { &*pac::Cmu::ptr() };

    // Enable peripheral clocks.
    cmu.hfperclkdiv().modify(|_, w| w.hfperclken().set_bit());
    cmu.hfperclken0().modify(|_, w| {
        w.timer0()
            .set_bit()
            .timer1()
            .set_bit()
            .acmp0()
            .set_bit()
            .prs()
            .set_bit()
    });

    let timer0 = unsafe { &*pac::Timer0::ptr() };
    let timer1 = unsafe { &*pac::Timer1::ptr() };

    // TIMER0: prescaler /512, top=10 (measurement window), interrupt on overflow.
    timer0.ctrl().write(|w| w.presc().div512());
    timer0.top().write(|w| unsafe { w.bits(10) });
    timer0.ien().write(|w| w.of().set_bit());
    timer0.cnt().write(|w| unsafe { w.bits(0) });

    // TIMER1: prescaler /1024, clock from CC1, top=0xFFFF.
    timer1.ctrl().write(|w| w.presc().div1024().clksel().cc1());
    timer1.top().write(|w| unsafe { w.bits(0xFFFF) });

    // TIMER1 CC1: input capture from PRS channel 0, both edges.
    timer1.cc1_ctrl().write(|w| {
        w.mode()
            .inputcapture()
            .prssel()
            .prsch0()
            .insel()
            .set_bit()
            .icedge()
            .both()
    });

    // PRS channel 0: ACMP0 capsense output, positive edge.
    let prs = unsafe { &*pac::Prs::ptr() };
    prs.ch0_ctrl()
        .write(|w| unsafe { w.edsel().posedge().sourcesel().bits(0x06).sigsel().bits(0) });

    // Set up ACMP0 in capsense mode.
    let acmp = unsafe { &*pac::Acmp0::ptr() };
    acmp.ctrl().write(|w| unsafe {
        w.bits(
            (7 << 24) // BIASPROG = 7
            | (5 << 8), // WARMTIME = 512 cycles
        )
    });

    // Enable TIMER0 interrupt.
    unsafe { pac::NVIC::unmask(pac::Interrupt::TIMER0) };
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

    // Disable PC1 (CAP1A) - Toboot enables it, interferes with ACMP.
    p.gpio.pc_model().modify(|_, w| w.mode1().disabled());

    setup_capsense();

    // Enable ACMP and start capsense.
    let acmp = unsafe { &*pac::Acmp0::ptr() };
    acmp.ctrl().modify(|r, w| unsafe { w.bits(r.bits() | 1) }); // EN bit
    CAPSENSE_RUNNING.store(true, Ordering::Release);
    capsense_measure(0);

    defmt::info!("Captouch coin flip ready");

    let mut last_gen: u32 = 0;
    let mut tick_count: u32 = 0;
    let mut coin_flip: u8 = 0;

    loop {
        // Wait for a new capsense measurement generation.
        let gen = CAPSENSE_GEN.load(Ordering::Relaxed);
        if gen == last_gen {
            cortex_m::asm::wfi();
            continue;
        }
        last_gen = gen;

        if coin_flip == 0 {
            let sum: u32 = (0..2)
                .map(|i| CHANNEL_VALUES[i].load(Ordering::Relaxed))
                .sum();
            if sum > CAPSENSE_DETECT_MIN {
                if tick_count.is_multiple_of(2) {
                    // Red = heads.
                    p.gpio.pb_doutclr().write(|w| unsafe { w.bits(1 << 7) });
                    coin_flip = 1;
                } else {
                    // Green = tails.
                    p.gpio.pa_doutclr().write(|w| unsafe { w.bits(1 << 0) });
                    coin_flip = 2;
                }
                tick_count = 0;
            }
        }

        tick_count += 1;

        if tick_count == LED_ON_TICKS {
            // LEDs off.
            p.gpio.pa_doutset().write(|w| unsafe { w.bits(1 << 0) });
            p.gpio.pb_doutset().write(|w| unsafe { w.bits(1 << 7) });
        } else if tick_count >= LED_OFF_TICKS {
            coin_flip = 0;
        }
    }
}
