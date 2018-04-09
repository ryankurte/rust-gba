use core::slice;
use core::ops::Index;
use core::ptr::{read_volatile, write_volatile};

pub const kb: usize = 1024;            // kilobit helper
pub const mb: usize = kb * 1024;       // megabit helper

// GBA Memory Sections (address and size)
pub const EWRAM:    (usize, usize) = (0x02000000, 256 * kb);  // 256kb of 16-bit wide External Working RAM (for application use)
pub const IWRAM:    (usize, usize) = (0x03000000, 32 * kb);   // 32 kb of 32-bit wide Internal Working RAM (for application use)
pub const IORAM:    (usize, usize) = (0x04000000, 1 * kb);    // 1kb of 16-bit Memory Mapped IO Registers
pub const PALRAM:   (usize, usize) = (0x05000000, 1 * kb);    // 1kb of 16-bit memory for colour pallets
pub const VRAM:     (usize, usize) = (0x06000000, 96 * kb);   // 96kb of 16-bit Video RAM for sprites and bitmaps
pub const OAM:      (usize, usize) = (0x07000000, 1 * kb);    // 1kb of 32-bit Object Attribute Memory, used for control of sprite rendering
pub const PAKROM:   (usize, usize) = (0x08000000, 32 * mb);   // Up to 32MB of 16-bit Game Pak ROM
pub const CARTRAM:  (usize, usize) = (0x0E000000, 64 * kb);   // Up to 64k of Cartridge ram for storing saved data

pub const REG_DISPCNT:  usize = IORAM.0 + 0x0000;   // Display control register
pub const REG_DISPSTAT: usize = IORAM.0 + 0x0004;   // Display status register
pub const REG_VCOUNT:   usize = IORAM.0 + 0x0006;   // Display scanline counter register

// Region helper wraps regions of a given type in volatile read and writes
#[derive(Debug, PartialEq)]
pub struct Region<T: 'static> (&'static mut[T]);

impl <T>From<(usize, usize)> for Region<T> {
    fn from(v: (usize, usize)) -> Region<T> {
        Region::new(v.0, v.1)
    }
}

impl <T>Region<T> {
    // New creates a new indexable memory region
    pub fn new(addr: usize, len: usize) -> Region<T> {
        unsafe {
            let data : &mut [T] = slice::from_raw_parts_mut(addr as *mut T, len);
            Region::<T>(data)
        }
    }
    
    pub fn read_index(&self, i: usize) -> &T {
        &self.0[i]
    }
    pub fn write_index(&mut self, i: usize, v: T) {
        self.0[i] = v;
    }

    pub fn read_addr(addr: u32) -> T {
        unsafe {
            read_volatile(addr as *const T)
        }
    }
    pub fn write_addr(addr: u32, v: T) {
        unsafe {
            write_volatile(addr as *mut T, v)
        }
    }
    
}

// Register helper structure
#[derive(Debug, PartialEq)]
pub struct Register<T> (*mut T);

impl <T>Register<T> {
    pub fn new(addr: usize) -> Register<T> {
        let v = addr as *mut T;
        Register::<T>(v)
    }
}

pub trait BitOps {
    fn read_bit(&self, i: usize) -> bool;
    fn write_bit(&mut self, i: usize, v: bool);
    fn read_masked(&self, shift: usize, mask: usize) -> usize;
    fn write_masked(&mut self, shift: usize, mask: usize, val: usize);
}

impl BitOps for Register<u32> {
    fn read_bit(&self, i: usize) -> bool {
        unsafe {
            *self.0 & (1 << i) != 0
        }
    }
    fn write_bit(&mut self, i: usize, v: bool) {
        unsafe {
            match v {
                true => *self.0 |= 1 << i,
                false => *self.0 &= !(1 << i),
            }
        }
    }
    fn read_masked(&self, shift: usize, mask: usize) -> usize {
        unsafe {
            (*self.0 as usize >> shift) & mask
        }
    }

    fn write_masked(&mut self, shift: usize, mask: usize, val: usize) {
        unsafe {
            *self.0 = ((*self.0 as usize & !(mask << shift)) | ((val & mask) << shift)) as u32;
        }
    }
}

impl BitOps for Register<u16> {
    fn read_bit(&self, i: usize) -> bool {
        unsafe {
            *self.0 & (1 << i) != 0
        }
    }
    fn write_bit(&mut self, i: usize, v: bool) {
        unsafe {
            match v {
                true => *self.0 |= 1 << i,
                false => *self.0 &= !(1 << i),
            }
        }
    }
    fn read_masked(&self, shift: usize, mask: usize) -> usize {
        unsafe {
            (*self.0 as usize >> shift) & mask
        }
    }
    fn write_masked(&mut self, shift: usize, mask: usize, val: usize) {
        unsafe {
            *self.0 = ((*self.0 as usize & !(mask << shift)) | ((val & mask) << shift)) as u16;
        }
    }
}

