#![no_main]
#![no_std]

use tfl as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    loop {}
}
