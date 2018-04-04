// Bitmap rendering modes

use core::slice;

use ::memory::{Region, VRAM, IORAM};
use ::graphics::colour::Colour;


const MODE3: (usize, usize)  = (240, 160);  // Mode 3, 240x160@16bpp single buffer
const MODE4: (usize, usize)  = (240, 160);  // Mode 4, 240x160@8bpp (pallet lookup) with swap
const MODE5: (usize, usize)  = (160, 128);  // Mode 5, 160x128@16bpp with swap

pub trait BitmapMode<T> {
    fn new() -> Self;
    fn bounds(&self) -> (usize, usize);
    fn enable(&mut self);
    fn swap(&mut self);
    fn set(&mut self, x: usize, y: usize, c: T);
}

#[derive(Debug, PartialEq)]
pub struct Mode3 {
    ioram: Region<u16>,
    vram: Region<u16>
}

impl BitmapMode<u16> for Mode3 {
    // Create a new mode3 instance
    fn new() -> Mode3 {
        Mode3{
            ioram: Region::from(IORAM),
            vram: Region::from(VRAM),
        }
    }

    // Get mode3 bounds
    fn bounds(&self) -> (usize, usize) {
        MODE3
    }

    fn swap(&mut self) {

    }

    // Enable mode 3
    fn enable(&mut self) {
        self.ioram.write_index(0, 0x0403);
    }

    // Set pixel value for mode 3
    fn set(&mut self, x: usize, y: usize, c: u16) {
        self.vram.write_index(x+y*MODE3.0, c);
    }
}

#[derive(Debug, PartialEq)]
pub struct Mode4 {
    ioram: Region<u16>,
    vram: [Region<u16>; 2],
    active: usize,
}

impl BitmapMode<u8> for Mode4 {
    // Create a new mode3 instance
    fn new() -> Mode4 {
        Mode4{
            ioram: Region::from(IORAM),
            vram: [
                Region::new(VRAM.0 + 0x0000, MODE4.0 * MODE4.1 * 16), 
                Region::new(VRAM.0 + 0xA000, MODE4.0 * MODE4.1 * 16),
            ],
            active: 0,
        }
    }

    // Get mode3 bounds
    fn bounds(&self) -> (usize, usize) {
        MODE3
    }

    // Enable mode 3
    fn enable(&mut self) {
        self.ioram.write_index(0, 0x0403);
    }

    fn swap(&mut self) {
        self.active = match self.active {
            0 => { 1 },
            _ => { 0 },
        };
    }

    // Set pixel value for mode 3
    // Note that VRAM can only be written in 16-bit chunks
    fn set(&mut self, x: usize, y: usize, c: u8) {
        let i = x + y * MODE3.0;
        let mut v: u16 = *self.vram[self.active].read_index(i / 2);
        v = if i % 2 == 0 {
            (v & 0xFF00) | c as u16
        } else {
            (v & 0x00FF) | ((c as u16) << 8)
        };
        self.vram[self.active].write_index(i / 2, v);
    }
}