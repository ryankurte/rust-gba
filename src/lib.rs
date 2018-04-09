// ryankurte/rust-gba
// Copyright 2018 Ryan Kurte

#![feature(lang_items)]
#![feature(used)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![no_std]

extern crate compiler_builtins;

use core::ptr;

pub mod header;
use header::{Header, LOGO};

pub mod memory;
pub mod graphics;
pub mod input;

// ARM 32-bit boot code 
// This sets the interrupt and app stack pointers and switches to thumb mode
// (linked at .text.boot prior to .text.reset_handler)
global_asm!(include_str!("gba_crt0.s"));

// Reset handler
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
global_asm!(include_str!("gba_isr.s"));

/// Handle interrupts from a thumb context
#[no_mangle]
#[used]
pub unsafe extern "C" fn isr_master() {
    // TODO: read interrupt registers and handle
}

// Default ROM header
#[link_section = ".header.header"]
#[used]
static HEADER: Header = Header {
    start_code: 0xEA00002E,     // Jump to start_code2 (0x080000c0)
    logo: LOGO,
    title: [0u8; 12],
    game_code: 0x00000000,
    maker_code: 0x3130,
    fixed: 0x96,
    unit_code: 0x00,
    device_type: 0x80,
    unused: [0u8; 0x07],
    game_version: 0x00,
    complement: 0x00,
    checksum: 0x0000,
    start_code2: 0xEA000008,    // Jump to _boot (0x080000e0)
    boot_method: 0x00,
    slave_number: 0x00,
    reserved: [0u8; 26],
};

// Rust no-std stubs

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(
    _args: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _col: u32,
) -> ! {
    loop {}
}

#[lang = "start"]
extern "C" fn start<T>(user_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize
where
    T: Termination,
{
    user_main().report() as isize
}

#[lang = "termination"]
trait Termination {
    fn report(self) -> i32;
}

impl Termination for () {
    fn report(self) -> i32 {
        0
    }
}