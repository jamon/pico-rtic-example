#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

// use bsp::hal::{
//     clocks::{init_clocks_and_plls, Clock},
//     pac,
//     sio::Sio,
//     watchdog::Watchdog,
// };
#[entry]
fn main() -> ! {
    info!("Program start");
    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    loop {
        info!("loop");
        // delay.delay_ms(500);

        // your code goes here
    }
}
