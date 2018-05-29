// Tile map rendering modes

use core::slice;

use memory::{VRAM, IORAM, REG_DISPCNT, DISPCNT};
use graphics::colour::Colour;

use embedded_builder::register::Register;
use embedded_builder::region::Region;

struct Mode6 {

}
