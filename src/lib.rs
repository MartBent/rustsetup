   
#![no_main]
#![no_std]

use tfl as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    loop {}
}