//! Graphics helpers

use crate::graphics::bitmap::{BitmapMode, PalletMode};
use crate::graphics::colour::{self, Colour};

/// The colours of a rainbow
const RAINBOW: [Colour; 6] = [
    colour::R,
    colour::O,
    colour::Y,
    colour::G,
    colour::B,
    colour::V,
];

/// Draw a rainbow with the given `thickness` to the graphic instance `g`.
pub fn rainbow<T: BitmapMode<u16>>(g: &mut T, thickness: usize) {
    let (w, h, _) = g.bounds();
    let start = h / 2 - RAINBOW.len() * thickness / 2;

    for x in 0..w {
        for c in 0..RAINBOW.len() {
            let offset = start + thickness * c;
            for y in offset..offset + thickness {
                g.set(x, y, RAINBOW[c].u16());
            }
        }
    }
}

/// Draw a rainbow with the given `thickness` to the graphic instance `g`,
/// where `g` is a pallet mode.
pub fn rainbow2<T: BitmapMode<u8> + PalletMode<u16>>(g: &mut T, thickness: usize) {
    // Set up pallet with rainbow colours
    for i in 0..RAINBOW.len() {
        g.set_pallet(i + 1, RAINBOW[i].u16())
    }

    {
        let (w, h, _) = g.bounds();
        let start = h / 2 - RAINBOW.len() * thickness / 2;

        for x in 0..w {
            for c in 0..RAINBOW.len() {
                let offset = start + thickness * c;
                for y in offset..offset + thickness {
                    g.set(x, y, (c + 1) as u8);
                }
            }
        }
    }
}
