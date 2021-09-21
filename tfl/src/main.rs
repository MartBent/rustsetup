#![no_main]
#![no_std]

use tfl as _;
use mock_io::MockIo;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");
    let x = MockIo{value: 0};
    let y = MockIo::default();

    x.write(5);
    y.read();
    
    loop {}
}
