// Graphics implementation
// ryankurte/rust-gba
// Copyright 2018 Ryan Kurte

pub mod colour;
pub mod bitmap;
pub mod tilemap;

use graphics::bitmap::{BitmapMode, Mode3};

#[derive(Debug, PartialEq)]
pub enum Mode {
    NONE,
    MODE1,
    MODE2,
    MODE3
}

pub enum None {}

pub struct Graphics<T> {
    mode: T
}

// Methods for all graphics implementations
impl <T>Graphics<T> {
    // Fetch active graphics mode
    pub fn active(&mut self) -> &mut T {
        &mut self.mode
    }
    
    // Set graphics to mode 3
    pub fn mode3(mut self) -> Graphics<bitmap::Mode3> {
        Graphics::new()
    }
}

// Methods for bitmap::Mode3 graphics
impl Graphics<bitmap::Mode3> {
    // Create a new Mode3 graphics instance
    pub fn new() -> Graphics<bitmap::Mode3> {
        let mut mode = bitmap::Mode3::new();
        mode.enable();
        Graphics::<bitmap::Mode3>{mode: mode}
    }
}