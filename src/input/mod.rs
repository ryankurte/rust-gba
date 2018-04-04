
use ::memory::{Region, VRAM, IORAM};

pub struct IO {
    ioram: Region<u16>
}

impl IO {
    pub fn new() -> IO {
        IO{ ioram: Region::from(IORAM) }
    }

    pub fn read() -> u16 {
        0u16
    }
}

