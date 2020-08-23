//! Tile map rendering modes

use crate::memory::{KB, PALRAM, VRAM};
use embedded_builder::region::Region;

pub trait Tile {}

pub type TileSingle = [u8; 32];
pub type TileDouble = [u8; 64];

pub struct Mode6 {
    background_blocks: [Region<TileSingle>; 4],
    sprite_blocks: [Region<TileSingle>; 2],
    _background_pallet: Region<u16>,
    _sprite_pallet: Region<u16>,
}

impl Mode6 {
    pub fn new() -> Mode6 {
        Mode6 {
            background_blocks: [
                Region::new(VRAM.0 + 0 * KB, 16 * KB),
                Region::new(VRAM.0 + 16 * KB, 16 * KB),
                Region::new(VRAM.0 + 32 * KB, 16 * KB),
                Region::new(VRAM.0 + 48 * KB, 16 * KB),
            ],
            sprite_blocks: [
                Region::new(VRAM.0 + 64 * KB, 16 * KB),
                Region::new(VRAM.0 + 80 * KB, 16 * KB),
            ],
            _background_pallet: Region::new(PALRAM.0 + 0x0000, 256),
            _sprite_pallet: Region::new(PALRAM.0 + 0x0200, 256),
        }
    }

    pub fn load_tile_background(&mut self, block: usize, id: usize, t: TileSingle) {
        self.background_blocks[block].write_index(id * 32, t)
    }

    pub fn load_tile_sprite(&mut self, block: usize, id: usize, t: TileSingle) {
        self.sprite_blocks[block].write_index(id * 32, t)
    }
}
