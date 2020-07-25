//! Gameboy Advance support for Rust

// ryankurte/rust-gba
// Copyright 2018 Ryan Kurte

#![no_std]
#![feature(lang_items)]
#![feature(asm, global_asm)]
#![feature(const_fn)]
#![feature(associated_type_defaults)]
#![allow(dead_code, unused_variables)]

extern crate embedded_builder;
extern crate gba;

use core::ptr;

pub mod graphics;
pub mod header;
pub mod input;
pub mod memory;

// ARM 32-bit boot code
// This sets the interrupt and app stack pointers and switches to thumb mode
// (linked at .text.boot prior to .text.reset_handler)
#[cfg(target_os = "none")]
global_asm!(include_str!("gba_crt0.s"));

/// Reset handler
#[cfg(target_os = "none")]
#[link_section = ".text.reset_handler"]
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> isize;

        static mut _sbss: u32;
        static mut _ebss: u32;

        static mut _sdata: u32;
        static mut _edata: u32;
        static _sidata: u32;

        static mut __sp_irq: u32;
        static mut __sp_usr: u32;
    }

    zero_bss(&mut _sbss, &mut _ebss);
    initialize_data(&mut _sdata, &mut _edata, &_sidata);

    main(0, ptr::null());

    loop {}
}

/// Clears the bss (uninitialized memory) section
unsafe fn zero_bss(sbss: *mut u32, ebss: *mut u32) {
    let mut bss = sbss;
    while bss < ebss {
        // NOTE(ptr::write*) to force aligned stores
        // NOTE(volatile) to prevent the compiler from optimizing this into `memclr`
        ptr::write_volatile(bss, 0);
        bss = bss.offset(1);
    }
}

/// Copies initial values in .data section from ROM to RAM
unsafe fn initialize_data(sdata: *mut u32, edata: *mut u32, sidata: *const u32) {
    let mut data = sdata;
    let mut idata = sidata;
    while data < edata {
        // NOTE(ptr::{read,write}) to force aligned loads and stores
        ptr::write(data, ptr::read(idata));
        data = data.offset(1);
        idata = idata.offset(1);
    }
}

// ARM 32-bit isr code
// This pushes to the stack then jumps to isr_master in thumb mode
// And returns from the interrupt following execution
#[cfg(target_os = "none")]
global_asm!(include_str!("gba_isr.s"));

/// Handle interrupts from a thumb context
#[cfg(target_os = "none")]
#[no_mangle]
pub unsafe extern "C" fn isr_master() {
    // TODO: read interrupt registers and handle
}

/// ROM header
#[cfg_attr(target_os = "none", link_section = ".header.header")]
#[used]
static HEADER: self::header::Header = self::header::Header::default();

/// Panic handler stub
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "none")]
#[lang = "start"]
extern "C" fn start<T>(user_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
where
    T: Termination,
{
    user_main().report() as isize
}

#[cfg(target_os = "none")]
#[lang = "termination"]
trait Termination {
    fn report(self) -> i32;
}

#[cfg(target_os = "none")]
impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}
