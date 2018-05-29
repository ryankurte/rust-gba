use core::slice;
use core::ops::{Index, Add, Sub, Not, BitAnd, BitOr, Shl, Shr, BitAndAssign, BitOrAssign};
use core::ptr::{read_volatile, write_volatile};

use embedded_builder;
use embedded_builder::register::Register;

pub const KB: usize = 1024;            // kilobit helper
pub const MB: usize = KB * 1024;       // megabit helper

// GBA Memory Sections (address and size)
pub const EWRAM:    (usize, usize) = (0x02000000, 256 * KB);  // 256KB of 16-bit wide External Working RAM (for application use)
pub const IWRAM:    (usize, usize) = (0x03000000, 32 * KB);   // 32 KB of 32-bit wide Internal Working RAM (for application use)
pub const IORAM:    (usize, usize) = (0x04000000, 1 * KB);    // 1KB of 16-bit Memory Mapped IO Registers
pub const PALRAM:   (usize, usize) = (0x05000000, 1 * KB);    // 1KB of 16-bit memory for colour pallets
pub const VRAM:     (usize, usize) = (0x06000000, 96 * KB);   // 96KB of 16-bit Video RAM for sprites and bitmaps
pub const OAM:      (usize, usize) = (0x07000000, 1 * KB);    // 1KB of 32-bit Object Attribute Memory, used for control of sprite rendering
pub const PAKROM:   (usize, usize) = (0x08000000, 32 * MB);   // Up to 32MB of 16-bit Game Pak ROM
pub const CARTRAM:  (usize, usize) = (0x0E000000, 64 * KB);   // Up to 64k of Cartridge ram for storing saved data

pub const REG_DISPCNT:  usize = IORAM.0 + 0x0000;   // Display control register
pub const REG_DISPSTAT: usize = IORAM.0 + 0x0004;   // Display status register
pub const REG_VCOUNT:   usize = IORAM.0 + 0x0006;   // Display scanline counter register
pub const REG_KEYINPUT: usize = IORAM.0 + 0x0130;   // Key input register

// Display control register traits
register!(DISPCNT, u16, 
    [
        r, mode,         1,  u16,   0,  0b111;  // Display mode read
        w, set_mode,     1,  u16,   0,  0b111;  // Display mode write
        r, is_gbc_cart,  1,  bool,  3;          // Check if cartrige is a GBC game
        w, set_ps,       1,  bool,  4;          // Page Select Write
        w, set_hb,       1,  bool,  5;          // Allow access to OAM during HBlank
        w, set_om,       1,  bool,  6;          // Object mapping mode (0=2d, 1=1d)
        w, force_blank,  1,  bool,  7;          // Force screen blank
        w, enable_bg0,   1,  bool,  8;          // Enable background 0 rendering
        w, enable_bg1,   1,  bool,  9;          // Enable background 1 rendering
        w, enable_bg2,   1,  bool,  10;         // Enable background 2 rendering
        w, enable_bg3,   1,  bool,  11;         // Enable background 3 rendering
        w, enable_obj,   1,  bool,  12;         // Enable object rendering
    ]
);

register!(DISPSTAT, u16,
    [
        r, vblank_status, 1, bool, 0; // VBlank Status (set in VBlank, cleared in VDraw)
        r, hblank_status, 1, bool, 1; // HBlank status (set in HBlank)
        r, vcount_status, 1, bool, 2; // VBlank status (set when scanline == VCount trigger value)
        r, vblank_irq_status, 1, bool, 3;
        w, vblank_irq_enable, 1, bool, 3;
        r, hblank_irq_status, 1, bool, 4;
        w, hblank_irq_enable, 1, bool, 4;
        r, vcount_irq_status, 1, bool, 5;
        w, vcount_irq_enable, 1, bool, 5;
        r, vcount_trigger_get, 1, u16, 6,  0xFF;  // Get VCount interrupt value
        w, vcount_trigger_set, 1, u16, 6,  0xFF;  // Set VCount interrupt value
    ]
);

// Key input register traits
register!(KEYINPUT, u16, 
    [
        r, a, 1,      bool,   0;
        r, b, 1,      bool,   1;
        r, select, 1, bool,   2;
        r, start, 1,  bool,   3;
        r, right, 1,  bool,   4;
        r, left, 1,   bool,   5;
        r, up, 1,     bool,   6;
        r, down, 1,   bool,   7;
        r, r, 1,      bool,   8;
        r, l, 1,      bool,   9;
    ]
);
