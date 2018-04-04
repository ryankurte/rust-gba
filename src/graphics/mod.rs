// Graphics implementation
// ryankurte/rust-gba
// Copyright 2018 Ryan Kurte

pub mod colour;
pub mod bitmap;
pub mod tilemap;

#[derive(Debug, PartialEq)]
pub enum Graphics {
    MODE1,
    MODE2,
    MODE3(bitmap::Mode3)
}



