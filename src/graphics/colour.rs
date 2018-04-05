// rust-gba
// 16-bit colour implementation

#[derive(Clone, Debug, PartialEq)]
pub struct Colour (u16);

// Helper macro to build static colour definitions
macro_rules! rgb16 {
    ($r:expr, $g:expr, $b:expr) => ((($r & 0x1F) as u16) | 
                                    ((($g & 0x1F) as u16) << 5) |
                                    ((($b & 0x1F) as u16) << 10))
}

// 16-bit colour (5 bits per channel)
impl Colour {
    pub const MAX: u8 = 0x1F;

    // Create a new colour from 16-bit representation
    pub fn new(v: u16) -> Colour {
        Colour(v)
    }

    // Create a new colour from individual channels
    pub fn rgb(red: u8, green: u8, blue: u8) -> Colour {
        Colour(rgb16!(red, green, blue))
    }

    // Fetch u16 representation
    pub fn u16(&self) -> u16 {
        self.0
    }

    // Helper colours
    pub const R: Colour = Colour(rgb16!(Self::MAX, 0, 0));
    pub const O: Colour = Colour(rgb16!(Self::MAX, Self::MAX/2, 0));
    pub const Y: Colour = Colour(rgb16!(Self::MAX, Self::MAX, 0));
    pub const G: Colour = Colour(rgb16!(0, Self::MAX, 0));
    pub const B: Colour = Colour(rgb16!(0, 0, Self::MAX));
    pub const V: Colour = Colour(rgb16!(Self::MAX/2, 0, Self::MAX));
}

