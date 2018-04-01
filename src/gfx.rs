// Graphics implementation
// ryankurte/rust-gba
// Copyright 2018 Ryan Kurte

use core::slice;

pub const IORAM: u32 = 0x04000000;
pub const VRAM: u32 = 0x06000000;

pub enum Graphics {
    MODE1,
    MODE2,
    MODE3
}

pub struct Colour (u16);

impl Colour {
    pub fn new(v: u16) -> Colour {
        Colour(v)
    }

    // Creates a colour for mode3 with 16 bits, 5 bits for each channel.
    pub fn rgb(red: u8, green: u8, blue: u8) -> Colour {
        Colour(
            ((red & 0x1F) as u16) | 
            (((green & 0x1F) as u16) << 5) |
            (((blue & 0x1F) as u16) << 10)
        )
    }
}

pub struct Mode3 {
    
}

const MODE3_W: usize = 240;
const MODE3_H: usize = 160;

impl Mode3 {
    // Create a new mode3 instance
    pub fn new() -> Mode3 {
        Mode3{}
    }

    // Enable mode 3
    pub fn enable(&self) {
        unsafe {
            *(IORAM as *mut u32)= 0x0403;
        }
    }

    // Get mode3 bounds
    pub fn bounds(&self) -> (usize, usize) {
        (MODE3_W, MODE3_H)
    }

    // Set pixel value for mode 3
    pub fn set(&self, x: usize, y: usize, c: Colour) {
        unsafe {
            let buff : &mut [u16] = slice::from_raw_parts_mut(VRAM as *mut u16, MODE3_W * MODE3_H);
            buff[x+y*MODE3_W] = c.0;
        }
    }
}

