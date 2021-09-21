
use tfl as _;

pub struct MockIo{
    value: i32
}

impl MockIo{
    pub fn read(self) -> i32{
        return self.value;
    }

    pub fn write(mut self, value: i32) {
        self.value = value;
    }
}