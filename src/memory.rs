use core::slice;
use core::ops::{Index, Add, Sub, Not, BitAnd, BitOr, Shl, Shr, BitAndAssign, BitOrAssign};
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


pub trait Zero {
    fn zero() -> Self;
}

impl Zero for u16 {
    fn zero() -> u16 {
        0
    }
}

impl Zero for u32 {
    fn zero() -> u32 {
        0
    }
}

pub trait One {
    fn one() -> Self;
}

impl One for u16 {
    fn one() -> u16 {
        1
    }
}

impl One for u32 {
    fn one() -> u32 {
        1
    }
}


// Unsigned integer trait
pub trait UnsignedInt<T>: Zero + One
                    + Not<Output=T> + Add<T, Output=T> + Sub<T, Output=T>
                    + BitAnd<T, Output=T> + BitOr<T, Output=T> + BitAndAssign<T> + BitOrAssign<T> 
                    + Shl<T, Output=T> + Shr<T, Output=T>
                    + Clone + Default + PartialEq {
                        type Output = T;
                    }

impl UnsignedInt<u16> for u16 {}
impl UnsignedInt<u32> for u32 {}

// Register helper structure
// This uses an internal value and builder approach to simplify interacting with registers.
#[derive(Debug, PartialEq, Clone)]
pub struct Register<T: UnsignedInt<T>> (usize, T);

impl <T: UnsignedInt<T>>Register<T> {
    // new creates a new register of the provided type with the specified address
    // Note that `impl UnsignedInt<T> for T {}` is required for unimplemented types
    pub fn new(addr: usize) -> Register<T> {
        Register(addr, T::default())
    }

    // u16 creates a new 16-bit ride register
    pub fn u16(addr: usize) -> Register<u16> {
        Register::<u16>::new(addr)
    }

    // u32 creates a new 32-bit register
    pub fn u32(addr: usize) -> Register<u32> {
        Register::<u32>::new(addr)
    }

    // read reads the register value and returns a new instance with
    // internal value set.
    pub fn read(&mut self) -> Register<T> {
        let mut reg = self.clone();
        unsafe {
            reg.1 = read_volatile(self.0 as *const T)
        }
        reg
    }

    // zero clears the internal register value
    pub fn zero(&mut self) -> Register<T>  {
        let mut reg = self.clone();
        reg.1 = T::zero();
        reg
    }

    // value returns the register value
    pub fn value(&self) -> T {
        self.1.clone()
    }

    // set sets the internal value of the register
    pub fn set(mut self, val: T) -> Register<T>  {
        self.1 = val;
        self
    }

    // and boolean and the provided and current values
    pub fn and(mut self, val: T) -> Register<T> {
        self.1 = self.1 & val;
        self
    }

    // or ors the provided and current values
    pub fn or(mut self, val: T) -> Register<T> {
        self.1 |= val;
        self
    }

    // clear clears the masked area of the provided value
    pub fn clear(mut self, mask: T) -> Register<T> {
        self.1 &= !mask;
        self
    }

    // get_bit returns a boolean consisting to the indexed bit
    pub fn get_bit(&self, i: T) -> bool {
        self.1.clone() & (T::one() << i) != T::zero()
    }

    // set_bit sets a bit in the current value
    pub fn set_bit(mut self, i: T, v: bool) -> Register<T> {
        self.1 = match v {
            true => self.1 | (T::one() << i),
            false => self.1 & !(T::one() << i),
        };
        self
    }

    // get_masked fetches a value with the provided mask and shift
    // Note that shift is applied prior to masking, so mask should always start at 0b1
    pub fn get_masked(&self, shift: T, mask: T, val: T) -> T  {
        (self.1.clone() >> shift) & mask
    }

    // set_masked sets a value with a provided mask and shift
    // Note that mask is applied before shifting, so mask should always start at 0b1
    pub fn set_masked(mut self, shift: T, mask: T, val: T) -> Register<T>  {
        self.clear(mask.clone()).or((val & mask) << shift)
    }

    // write writes the internal value to the register
    pub fn write(self) {
        unsafe {
            write_volatile(self.0 as *mut T, self.1)
        }
    }
}

pub trait RegisterOps<T> {
    fn mask(self, mask: T) -> Self;
}


pub trait BitOps {
    fn read_bit(&self, i: usize) -> bool;
    fn write_bit(&mut self, i: usize, v: bool);
    fn read_masked(&self, shift: usize, mask: usize) -> usize;
    fn write_masked(&mut self, shift: usize, mask: usize, val: usize);
}
