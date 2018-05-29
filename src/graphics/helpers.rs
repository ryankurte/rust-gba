
use ::graphics::colour::Colour;
use ::graphics::bitmap::{BitmapMode, PalletMode};

pub trait Rainbow<T> {
    fn rainbow(&mut T);
}

const RAINBOW: [Colour; 6] = [
    Colour::R,
    Colour::O,
    Colour::Y,
    Colour::G,
    Colour::B,
    Colour::V,
];

pub fn rainbow<T: BitmapMode<u16>>(g: &mut T, thickness: usize) {
    let (w, h, _) = g.bounds();
    let start = h / 2 - RAINBOW.len() * thickness / 2;

    for x in 0..w {
        for c in 0..RAINBOW.len() {
            let offset = start + thickness * c;
            for y in offset..offset+thickness {
                g.set(x, y, RAINBOW[c].u16());
            }
        }
    }
}

pub fn rainbow2<T: BitmapMode<u8> + PalletMode<u16>>(g: &mut T, thickness: usize) {

    // Set up pallet with rainbow colours
    for i in 0..RAINBOW.len() {
        g.set_pallet(i+1, RAINBOW[i].u16())
    }

    {
        let (w, h, _) = g.bounds();
        let start = h / 2 - RAINBOW.len() * thickness / 2;

        for x in 0..w {
            for c in 0..RAINBOW.len() {
                let offset = start + thickness * c;
                for y in offset..offset+thickness {
                    g.set(x, y, (c + 1) as u8);
                }
            }
        }
    }

}

