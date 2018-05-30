// Tile map rendering modes

use core::slice;

use memory::{KB, VRAM, PALRAM, REG_DISPCNT, DISPCNT};
use graphics::colour::Colour;

use embedded_builder::register::Register;
use embedded_builder::region::Region;

trait Tile {

}

type TileSingle = [u8; 32];
type TileDouble = [u8; 64];



struct Mode6 {
    background_blocks:  [Region<TileSingle>; 4],
    sprite_blocks:      [Region<TileSingle>; 2],
    background_pallet:  Region<u16>,
    sprite_pallet:      Region<u16>,
}

impl Mode6 {
    fn new() -> Mode6 {
        Mode6{
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
            background_pallet: Region::new(PALRAM.0 + 0x0000, 256),
            sprite_pallet: Region::new(PALRAM.0 + 0x0200, 256),
        }
    }

    fn load_tile_background(&mut self, block: usize, id: usize, t: TileSingle) {
        self.background_blocks[block].write_index(id * 32, t)
    }

    fn load_tile_sprite(&mut self, block: usize, id: usize, t: TileSingle) {
        self.background_blocks[block].write_index(id * 32, t)
    }

}
