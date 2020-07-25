//! Memory sections

/// A kilobyte
pub const KB: usize = 1024;
/// A megabyte
pub const MB: usize = KB * 1024;

/*
 * GBA Memory Sections as `(address, size)`
 */

/// 256KB of 16-bit wide External Working RAM (for application use)
pub const EWRAM: (usize, usize) = (0x02000000, 256 * KB);
/// 32 KB of 32-bit wide Internal Working RAM (for application use)
pub const IWRAM: (usize, usize) = (0x03000000, 32 * KB);
/// 1KB of 16-bit Memory Mapped IO Registers
pub const IORAM: (usize, usize) = (0x04000000, 1 * KB);
/// 1KB of 16-bit memory for colour pallets
pub const PALRAM: (usize, usize) = (0x05000000, 1 * KB);
/// 96KB of 16-bit Video RAM for sprites and bitmaps
pub const VRAM: (usize, usize) = (0x06000000, 96 * KB);
/// 1KB of 32-bit Object Attribute Memory, used for control of sprite rendering
pub const OAM: (usize, usize) = (0x07000000, 1 * KB);
/// Up to 32MB of 16-bit Game Pak ROM
pub const PAKROM: (usize, usize) = (0x08000000, 32 * MB);
/// Up to 64k of Cartridge ram for storing saved data
pub const CARTRAM: (usize, usize) = (0x0E000000, 64 * KB);

/*
 * Registers
 */

/// Display control register
pub const REG_DISPCNT: usize = IORAM.0 + 0x0000;
/// Display status register
pub const REG_DISPSTAT: usize = IORAM.0 + 0x0004;
/// Display scanline counter register
pub const REG_VCOUNT: usize = IORAM.0 + 0x0006;
/// Key input register
pub const REG_KEYINPUT: usize = IORAM.0 + 0x0130;
