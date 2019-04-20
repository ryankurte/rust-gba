

use embedded_builder::register::Register;

use gba::io::keypad::KEYINPUT;

pub struct IO {
    state:   u16,
    changed: u16,
}

#[derive(Copy, Clone)]
pub enum Keys {
    A       = 1 << 0,
    B       = 1 << 1,
    Start   = 1 << 2,
    Select  = 1 << 3,
    Right   = 1 << 4,
    Left    = 1 << 5,
    Up      = 1 << 6,
    Down    = 1 << 7,
    L       = 1 << 8,
    R       = 1 << 9,
}

impl IO {
    /// Create a new IO interface
    pub fn new() -> IO {
        IO{ state: 0, changed: 0 }
    }

    /// Update the key information
    pub fn update(&mut self) {
        let state = !KEYINPUT.read();
        self.changed = state ^ self.state;
        self.state = state;
    }

    /// Check if a key is currently pressed
    pub fn is_pressed(&self, k: Keys) -> bool {
        let i: u16 = *&k as u16;
        self.state & i != 0
    }

    /// Check if a key has been toggled since the last button press
    pub fn toggled(&self, k: Keys) -> bool {
        let i: u16 = *&k as u16;
        self.changed & i != 0
    }

    /// Check if a key has been pressed since the last update
    pub fn pressed(&self, k: Keys) -> bool {
        self.is_pressed(k) && self.toggled(k)
    }

    /// Check if a key has been released since the last update
    pub fn released(&self, k: Keys) -> bool {
        !self.is_pressed(k) && self.toggled(k)
    }
}

