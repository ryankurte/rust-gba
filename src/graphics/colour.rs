

#[derive(Clone, Debug, PartialEq)]
pub struct Colour (u16);

impl Colour {
    pub fn new(v: u16) -> Colour {
        Colour(v)
    }

    // Creates a colour for mode3 with 16 bits, 5 bits for each channel.
    pub fn rgb(red: u8, green: u8, blue: u8) -> Colour {
        Colour(
            ((red & 0x1F) as u16) | 
            (((green & 0x1F) as u16) << 5) |
            (((blue & 0x1F) as u16) << 10)
        )
    }

    pub fn u16(&self) -> u16 {
        self.0
    }
}

