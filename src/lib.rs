   
#![no_main]
#![no_std]

use {{crate_name}} as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    loop {}
}