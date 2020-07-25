//! Bitmap rendering modes

use embedded_builder::region::Region;
use gba::io::display::{DisplayControlSetting, DisplayMode, DISPCNT, DISPSTAT};

use crate::memory::{IORAM, PALRAM, VRAM};

/// Mode 3, 240x160@16bpp single buffer
const MODE3: (usize, usize, usize) = (240, 160, 16);

/// Mode 4, 240x160@8bpp (pallet lookup) with swap
const MODE4: (usize, usize, usize) = (240, 160, 8);

/// Mode 5, 160x128@16bpp with swap
const MODE5: (usize, usize, usize) = (160, 128, 16);

/// Bitmap mode trait, implemented by bitmap rendering modes
pub trait BitmapMode<T> {
    /// Create a new instance of the rendering mode.
    fn new() -> Self;

    /// Return the bounds of the rendering mode, as a tuple of
    /// `(width, height, depth)`.
    fn bounds(&self) -> (usize, usize, usize);

    /// Enables the rendering mode.
    fn enable(&mut self);

    /// Swaps the buffers, for double-buffer rendering modes.
    fn swap(&mut self);

    /// Set a pixel at `(x, y)` to the colour `c`.
    ///
    /// This operates on the inactive buffer, for double-buffer rendering
    /// modes.
    fn set(&mut self, x: usize, y: usize, c: T);

    /// Clears the buffer.
    ///
    /// This operates on the inactive buffer, for double-buffer rendering
    /// modes.
    fn clear(&mut self);
}

/// Pallet mode trait, implemented by bitmap modes with pallet lookup
pub trait PalletMode<T> {
    /// Set the pallet index `i` to the colour `c`.
    fn set_pallet(&mut self, i: usize, c: T);

    /// Get the colour at the pallet index `i`.
    fn get_pallet(&self, i: usize) -> T;
}

/// Graphics mode 3 - 240x160@16bpp, single-buffer
///
/// Note: The `swap()` operation on a mode 3 graphics instance is a no-op.
#[derive(Debug, PartialEq)]
pub struct Mode3 {
    ioram: Region<u16>,
    vram: Region<u16>,
}

impl BitmapMode<u16> for Mode3 {
    fn new() -> Mode3 {
        Mode3 {
            ioram: Region::from(IORAM),
            vram: Region::new(VRAM.0 + 0x0000, MODE3.0 * MODE3.1 * MODE3.2),
        }
    }

    fn bounds(&self) -> (usize, usize, usize) {
        MODE3
    }

    fn swap(&mut self) {
        // Mode 3 is not double-buffered.
    }

    fn enable(&mut self) {
        let dispcnt = DisplayControlSetting::new().with_mode(DisplayMode::Mode3);
        DISPCNT.write(dispcnt);
    }

    fn set(&mut self, x: usize, y: usize, c: u16) {
        self.vram.write_index(x + y * MODE3.0, c);
    }

    fn clear(&mut self) {
        for x in 0..self.bounds().0 {
            for y in 0..self.bounds().1 {
                self.set(x, y, 0);
            }
        }
    }
}

/// Swap buffer enumeration
#[derive(Debug, PartialEq)]
enum SwapBuffer {
    A,
    B,
}

/// Graphics mode 4 - 240x160@8bpp, double-buffered
#[derive(Debug, PartialEq)]
pub struct Mode4 {
    ioram: Region<u16>,
    vram: [Region<u16>; 2],
    pallet: Region<u16>,
    active: SwapBuffer,
}

impl BitmapMode<u8> for Mode4 {
    fn new() -> Mode4 {
        Mode4 {
            ioram: Region::from(IORAM),
            vram: [
                Region::new(VRAM.0 + 0x0000, MODE4.0 * MODE4.1 * MODE4.2 / 8),
                Region::new(VRAM.0 + 0xA000, MODE4.0 * MODE4.1 * MODE4.2 / 8),
            ],
            pallet: Region::new(PALRAM.0, 256),
            active: SwapBuffer::A,
        }
    }

    fn bounds(&self) -> (usize, usize, usize) {
        MODE4
    }

    fn enable(&mut self) {
        let dispcnt = DisplayControlSetting::new()
            .with_mode(DisplayMode::Mode4)
            .with_bg2(true)
            .with_frame1(true);
        DISPCNT.write(dispcnt);
    }

    fn swap(&mut self) {
        let mut dispcnt = DISPCNT.read();
        self.active = match self.active {
            SwapBuffer::A => {
                dispcnt = dispcnt.with_frame1(true);
                SwapBuffer::B
            }
            SwapBuffer::B => {
                dispcnt = dispcnt.with_frame1(false);
                SwapBuffer::A
            }
        };
        DISPCNT.write(dispcnt);
    }

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

    fn clear(&mut self) {
        for x in 0..self.bounds().0 {
            for y in 0..self.bounds().1 {
                self.set(x, y, 0);
            }
        }
    }
}

impl PalletMode<u16> for Mode4 {
    fn set_pallet(&mut self, i: usize, c: u16) {
        self.pallet.write_index(i, c)
    }

    fn get_pallet(&self, i: usize) -> u16 {
        *self.pallet.read_index(i)
    }
}

impl Mode4 {
    pub fn vblank(&mut self) -> bool {
        DISPSTAT.read().vblank_flag()
    }
}

/// Graphics mode 5 - 160x128@16bpp, double-buffered
#[derive(Debug, PartialEq)]
pub struct Mode5 {
    ioram: Region<u16>,
    vram: [Region<u16>; 2],
    active: SwapBuffer,
}

impl BitmapMode<u16> for Mode5 {
    fn new() -> Mode5 {
        Mode5 {
            ioram: Region::from(IORAM),
            vram: [
                Region::new(VRAM.0 + 0x0000, MODE5.0 * MODE5.1 * MODE5.2 / 8),
                Region::new(VRAM.0 + 0xA000, MODE5.0 * MODE5.1 * MODE5.2 / 8),
            ],
            active: SwapBuffer::A,
        }
    }

    fn bounds(&self) -> (usize, usize, usize) {
        MODE5
    }

    fn enable(&mut self) {
        let dispcnt = DisplayControlSetting::new()
            .with_mode(DisplayMode::Mode5)
            .with_bg2(true)
            .with_frame1(true);
        DISPCNT.write(dispcnt);
    }

    fn swap(&mut self) {
        let mut dispcnt = DISPCNT.read();
        self.active = match self.active {
            SwapBuffer::A => {
                dispcnt = dispcnt.with_frame1(true);
                SwapBuffer::B
            }
            SwapBuffer::B => {
                dispcnt = dispcnt.with_frame1(false);
                SwapBuffer::A
            }
        };
        DISPCNT.write(dispcnt);
    }

    fn set(&mut self, x: usize, y: usize, c: u16) {
        match self.active {
            SwapBuffer::A => {
                self.vram[1].write_index(x + y * MODE5.0, c);
            }
            SwapBuffer::B => {
                self.vram[0].write_index(x + y * MODE5.0, c);
            }
        };
    }

    fn clear(&mut self) {
        for x in 0..self.bounds().0 {
            for y in 0..self.bounds().1 {
                self.set(x, y, 0);
            }
        }
    }
}
