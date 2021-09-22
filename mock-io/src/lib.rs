#![no_std]

use defmt::info;

pub struct MockIo{
    pub value: i32,
}

impl MockIo{

    pub fn default() -> MockIo {
        MockIo{value: 0}
    }

    pub fn read(&self) -> i32{
        info!("Reading IO...");
        return self.value + 1;
    }

    pub fn write(&mut self, value: i32) {
        info!("Writing {} to IO...", value);
        self.value = value;
    }
}
