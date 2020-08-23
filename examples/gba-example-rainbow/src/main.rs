#![no_std]
#![feature(asm)]

use gba_core::{
    graphics::{
        bitmap::{BitmapMode, Mode4},
        helpers::rainbow2,
        Graphics,
    },
    input::{Keys, IO},
};

fn main() {
    let mut io = IO::new();
    let mut m = Graphics::<Mode4>::new();
    let g = m.active();

    let mut width = 10;

    loop {
        io.update();

        if io.is_pressed(Keys::Up) && width < 20 {
            width += 1;
        } else if io.is_pressed(Keys::Down) && width > 2 {
            width -= 1;
        }

        g.clear();
        rainbow2(g, width);

        while g.vblank() {}
        g.swap();
    }
}
