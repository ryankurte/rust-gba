// Bitmap rendering modes

use memory::{VRAM, IORAM, PALRAM, REG_DISPCNT, DISPCNT, REG_DISPSTAT, DISPSTAT};

use embedded_builder::register::Register;
use embedded_builder::region::Region;

const MODE3: (usize, usize, usize)  = (240, 160, 16);  // Mode 3, 240x160@16bpp single buffer
const MODE4: (usize, usize, usize)  = (240, 160, 8);   // Mode 4, 240x160@8bpp (pallet lookup) with swap
const MODE5: (usize, usize, usize)  = (160, 128, 16);  // Mode 5, 160x128@16bpp with swap

#[doc = "Bitmap mode implemented by bitmap rendering modes"]
pub trait BitmapMode<T> {
    fn new() -> Self;
    fn bounds(&self) -> (usize, usize, usize);
    fn enable(&mut self);
    fn swap(&mut self);
    fn set(&mut self, x: usize, y: usize, c: T);
    fn clear(&mut self);
}

#[doc = "Pallet mode implemented by bitmap modes with pallet lookup"]
pub trait PalletMode<T> {
    fn set_pallet(&mut self, i: usize, c: T);
    fn get_pallet(&self, i: usize) -> T;
}

#[doc = "Mode 3 240x16 16-bit single buffered"]
#[derive(Debug, PartialEq)]
pub struct Mode3 {
    ioram: Region<u16>,
    vram: Region<u16>,
    display_control: Register<u16>,
}

impl BitmapMode<u16> for Mode3 {
    // Create a new mode3 instance
    fn new() -> Mode3 {
        Mode3{
            ioram: Region::from(IORAM),
            vram: Region::new(VRAM.0 + 0x0000, MODE3.0 * MODE3.1 * MODE3.2),
            display_control: Register::new(REG_DISPCNT),
        }
    }

    // Get mode3 bounds
    fn bounds(&self) -> (usize, usize, usize) {
        MODE3
    }

    // Swap buffers (not implemented in Mode3)
    fn swap(&mut self) {

    }

    // Enable mode 3
    fn enable(&mut self) {
        self.display_control.zero().set_mode(3).enable_bg2(true).write();
    }

    // Set pixel value for mode 3
    fn set(&mut self, x: usize, y: usize, c: u16) {
        self.vram.write_index(x+y*MODE3.0, c);
    }

    // Clear the display
    fn clear(&mut self) {
        for x in 0..self.bounds().0 {
            for y in 0..self.bounds().1 {
                self.set(x, y, 0);
            }
        }
    }
}

#[doc = "Swap buffer enumeration"]
#[derive(Debug, PartialEq)]
enum SwapBuffer {
    A,
    B
}

#[doc = "Mode 4 240x160 8-bit pallet lookup with swap buffer"]
#[derive(Debug, PartialEq)]
pub struct Mode4 {
    ioram: Region<u16>,
    vram: [Region<u16>; 2],
    pallet: Region<u16>,
    active: SwapBuffer,
    display_control: Register<u16>,
    display_status: Register<u16>,
}

impl BitmapMode<u8> for Mode4 {
    // Create a new mode4 instance
    fn new() -> Mode4 {
        Mode4{
            ioram: Region::from(IORAM),
            vram: [
                Region::new(VRAM.0 + 0x0000, MODE4.0 * MODE4.1 * MODE4.2 / 8), 
                Region::new(VRAM.0 + 0xA000, MODE4.0 * MODE4.1 * MODE4.2 / 8),
            ],
            pallet: Region::new(PALRAM.0, 256),
            active: SwapBuffer::A,
            display_control: Register::new(REG_DISPCNT),
            display_status: Register::new(REG_DISPSTAT),
        }
    }

    // Get mode4 bounds
    fn bounds(&self) -> (usize, usize, usize) {
        MODE4
    }

    // Enable mode 4
    fn enable(&mut self) {
        self.display_control.zero()
            .set_mode(4).enable_bg2(true).set_ps(false)
            .write();
    }

    // Swap active and inactive buffers
    fn swap(&mut self) {
        self.active = match self.active {
            SwapBuffer::A => { 
                self.display_control.read().set_ps(true).write();
                SwapBuffer::B 
            },
            SwapBuffer::B => { 
                self.display_control.read().set_ps(false).write();
                SwapBuffer::A 
            },
        };
    }

    // Set pixel index (for pallet lookup) for mode 4
    // This writes to the currently inactive buffer
    // Note that VRAM can only be written in 16-bit chunks
    fn set(&mut self, x: usize, y: usize, c: u8) {
        let i = x + y * MODE3.0;
        let vram = match self.active {
            SwapBuffer::A => &mut self.vram[1],
            SwapBuffer::B => &mut self.vram[0],
        };
        let mut v: u16 = *vram.read_index(i / 2);
        v = if i % 2 == 0 {
            (v & 0xFF00) | c as u16
        } else {
            (v & 0x00FF) | ((c as u16) << 8)
        };
        vram.write_index(i / 2, v);
    }

    // Clear the currently inactive buffer
    fn clear(&mut self) {
        for x in 0..self.bounds().0 {
            for y in 0..self.bounds().1 {
                self.set(x, y, 0);
            }
        }
    }
}

impl PalletMode<u16> for Mode4 {
    // Set pallet sets the pallet colour at a given index
    // These indices are used in the set function to specify colour
    fn set_pallet(&mut self, i: usize, c: u16) {
        self.pallet.write_index(i, c)
    }

    // Set pallet sets the pallet colour at a given index
    // These indices are used in the set function to specify colour
    fn get_pallet(&self, i: usize) -> u16 {
        *self.pallet.read_index(i)
    }
}

impl Mode4 {
    pub fn vblank(&mut self) -> bool {
        self.display_control.read().vblank_status()
    }
}

#[doc = "Mode 5 160x128 16-bit colour with swap buffer"]
#[derive(Debug, PartialEq)]
pub struct Mode5 {
    ioram: Region<u16>,
    vram: [Region<u16>; 2],
    active: SwapBuffer,
    display_control: Register<u16>,
}

impl BitmapMode<u16> for Mode5 {
    // Create a new mode5 instance
    fn new() -> Mode5 {
        Mode5{
            ioram: Region::from(IORAM),
             vram: [
                Region::new(VRAM.0 + 0x0000, MODE5.0 * MODE5.1 * MODE5.2 / 8), 
                Region::new(VRAM.0 + 0xA000, MODE5.0 * MODE5.1 * MODE5.2 / 8),
            ],
            active: SwapBuffer::A,
            display_control: Register::new(REG_DISPCNT),
        }
    }

    // Get mode5 bounds
    fn bounds(&self) -> (usize, usize, usize) {
        MODE5
    }

    // Enable mode 5
    fn enable(&mut self) {
        self.display_control.zero().set_mode(5).enable_bg2(true).write();
    }

    // Swap active and inactive buffers
    fn swap(&mut self) {
        self.active = match self.active {
            SwapBuffer::A => { 
                self.display_control.read().set_ps(true).write();
                SwapBuffer::B 
            },
            SwapBuffer::B => { 
                self.display_control.read().set_ps(false).write();
                SwapBuffer::A 
            },
        };
    }

    // Set pixel value for mode 5
    // This writes to the currently inactive buffer
    // Note that VRAM can only be written in 16-bit chunks
    fn set(&mut self, x: usize, y: usize, c: u16) {
        match self.active {
            SwapBuffer::A => { self.vram[1].write_index(x + y * MODE5.0, c); },
            SwapBuffer::B => { self.vram[0].write_index(x + y * MODE5.0, c); },
        };
    }

    // Clear the currently inactive buffer
    fn clear(&mut self) {
        for x in 0..self.bounds().0 {
            for y in 0..self.bounds().1 {
                self.set(x, y, 0);
            }
        }
    }
}