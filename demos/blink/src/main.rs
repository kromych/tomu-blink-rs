#![no_main]
#![no_std]

// Panic handler.
use efm32hg309_pac as pac;
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    // If the Watchdog is not reset, the board will reboot.
    peripherals.WDOG.ctrl.reset();

    loop {
        cortex_m::asm::wfi();
    }
}
