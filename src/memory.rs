use core::slice;
use core::ops::{Index, Add, Sub, Not, BitAnd, BitOr, Shl, Shr, BitAndAssign, BitOrAssign};
use core::ptr::{read_volatile, write_volatile};

#[macro_use]
use embedded_builder;

use embedded_builder::register::Register;

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

// Display control register traits
register!(DISPCNT, u16, 
    [
        r, mode,        1, u16,     0,  0b111;  // Display mode read
        w, set_mode,    1, u16,     0,  0b111;  // Display mode write
        r, is_gbc_cart, 1,  bool,   3;          // Check if cartrige is a GBC game
        w, write_ps,    1,  bool,   4;          // Page Select Write
        w, write_hb,    1,  bool,   5;          // Allow access to OAM during HBlank
        w, write_om,    1,  bool,   6;          // Object mapping mode (0=2d, 1=1d)
        w, force_blank, 1,  bool,   7;          // Force screen blank
        w, enable_bg0,   1,  bool,  8;         // Enable background 0 rendering
        w, enable_bg1,   1,  bool,  9;         // Enable background 1 rendering
        w, enable_bg2,   1,  bool,  10;        // Enable background 2 rendering
        w, enable_bg3,   1,  bool,  11;        // Enable background 3 rendering
        w, enable_obj,   1,  bool,  12;        // Enable object rendering
    ]
);
