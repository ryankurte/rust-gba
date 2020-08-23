//! Colours

/// Helper macro to build static colour definitions
macro_rules! rgb16 {
    ($r:expr, $g:expr, $b:expr) => {
        (($r & 0x1F) as u16) | ((($g & 0x1F) as u16) << 5) | ((($b & 0x1F) as u16) << 10)
    };
}

/// 16-bit colour (5 bits per channel)
#[derive(Clone, Debug, PartialEq)]
pub struct Colour(u16);

impl Colour {
    /// Maximum value for a colour
    pub const MAX: u8 = 0x1F;

    /// Create a new colour from 16-bit representation
    pub fn new(v: u16) -> Colour {
        Colour(v)
    }

    /// Create a new colour from individual channels
    pub fn rgb(red: u8, green: u8, blue: u8) -> Colour {
        Colour(rgb16!(red, green, blue))
    }

    /// Get this colour's 16-bit representation
    pub fn u16(&self) -> u16 {
        self.0
    }
}

/// Helper colour: Red
pub const R: Colour = Colour(rgb16!(Colour::MAX, 0, 0));

/// Helper colour: Orange
pub const O: Colour = Colour(rgb16!(Colour::MAX, Colour::MAX / 2, 0));

/// Helper colour: Yellow
pub const Y: Colour = Colour(rgb16!(Colour::MAX, Colour::MAX, 0));

/// Helper colour: Green
pub const G: Colour = Colour(rgb16!(0, Colour::MAX, 0));

/// Helper colour: Blue
pub const B: Colour = Colour(rgb16!(0, 0, Colour::MAX));

/// Helper colour: Violet
pub const V: Colour = Colour(rgb16!(Colour::MAX / 2, 0, Colour::MAX));

/// Helper colour: White
pub const W: Colour = Colour(rgb16!(Colour::MAX, Colour::MAX, Colour::MAX));

/// Helper colour: Black
pub const BL: Colour = Colour(rgb16!(0, 0, 0));
