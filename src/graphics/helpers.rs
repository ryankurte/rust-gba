
use ::graphics::colour::Colour;
use ::graphics::bitmap::BitmapMode;

pub fn rainbow<T: BitmapMode<u16>>(g: &mut T, thickness: usize) {
    let rainbow: [Colour; 6] = [
        Colour::R,
        Colour::O,
        Colour::Y,
        Colour::G,
        Colour::B,
        Colour::V,
    ];

    {
        let (w, h, _) = g.bounds();
        let start = h / 2 - rainbow.len() * thickness / 2;

        for x in 0..w {
            for c in 0..rainbow.len() {
                let offset = start + thickness * c;
                for y in offset..offset+thickness {
                    g.set(x, y, rainbow[c].u16());
                }
            }
        }
    }

}
