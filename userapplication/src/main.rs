#![no_main]
#![no_std]

use tfl as _;
use defmt::{info, error};

use stm32wl_hal::{embedded_time::rate::Hertz, gpio::{PortA}, i2c::I2c2, pac};

#[cortex_m_rt::entry]
fn main() -> ! {
    
    let mut dp = pac::Peripherals::take().unwrap();
    let gpioa = PortA::split(dp.GPIOA, &mut dp.RCC);

    let i2c = I2c2::new(dp.I2C2, (gpioa.a12, gpioa.a11), Hertz(100000), &mut dp.RCC, true);

    let mut sensor = scd30::scd30::Scd30::new_with_address(i2c, 0x61);

    info!("Starting measuring!");

    match sensor.start_measuring() {
        Err(e) => error!("{}",e),
        Ok(_) => info!("Started succesfully"),
    }

    match sensor.set_automatic_calibration(true) {
        Err(e) => error!("{}",e),
        Ok(_) => info!("ASC Set!"),
    }

    stm32wl_hal::cortex_m::asm::delay(16000000);

    loop {
        match sensor.read() {
            Ok(measurement) => {
                match measurement {
                    Some(measurement ) => info!("[Temperature: {}, Humidity: {}, co2: {}]", measurement.temperature, measurement.humidity, measurement.co2),
                    None => {}
                }
            }
            
            Err(e) => error!("{}", e)
        }

        stm32wl_hal::cortex_m::asm::delay(4000_000);
    }
}