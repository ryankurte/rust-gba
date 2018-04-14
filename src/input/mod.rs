

use embedded_builder::register::Register;
use embedded_builder::region::Region;

use ::memory::{VRAM, IORAM};

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

