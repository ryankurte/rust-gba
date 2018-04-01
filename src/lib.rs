#![feature(lang_items)]
#![feature(used)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(compiler_builtins_lib)]
#![no_std]

extern crate compiler_builtins;

use core::ptr;

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

// ARM 32-bit boot code (linked at .text.boot)
// TODO: split to file, though a given macro can seemingly only contain one ISA.
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

unsafe fn zero_bss(sbss: *mut u32, ebss: *mut u32) {
    let mut bss = sbss;
    while bss < ebss {
        // NOTE(ptr::write*) to force aligned stores
        // NOTE(volatile) to prevent the compiler from optimizing this into `memclr`
        ptr::write_volatile(bss, 0);
        bss = bss.offset(1);
    }
}

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

#[repr(C, packed)]
struct Header {
    start_code: u32,    // Trampoline 1, b 80000c0, jumps to end of header
    logo: [u8; 0x9C],
    title: [u8; 12],
    game_code: u32,
    maker_code: u16,
    fixed: u8,
    unit_code: u8,
    device_type: u8,
    unused: [u8; 7],
    game_version: u8,
    complement: u8,
    checksum: u16,
    start_code2: u32,   // Trampoline 2, b 80000e0, jumps to RESET_VECTOR
    boot_method: u8,    // 0 for ROM boot, 3 for multiplay
    slave_number: u8,   // Slave ID for multiplay boot
    reserved: [u8; 26],
    //boot: [u32; 8],
    //jump: [u32; 2],
}

// Nintendo logo (required)
const LOGO: [u8; 0x9C] = [
        0x24,0xFF,0xAE,0x51,0x69,0x9A,0xA2,0x21,0x3D,0x84,0x82,0x0A,0x84,0xE4,0x09,0xAD,
        0x11,0x24,0x8B,0x98,0xC0,0x81,0x7F,0x21,0xA3,0x52,0xBE,0x19,0x93,0x09,0xCE,0x20,
        0x10,0x46,0x4A,0x4A,0xF8,0x27,0x31,0xEC,0x58,0xC7,0xE8,0x33,0x82,0xE3,0xCE,0xBF,
        0x85,0xF4,0xDF,0x94,0xCE,0x4B,0x09,0xC1,0x94,0x56,0x8A,0xC0,0x13,0x72,0xA7,0xFC,
        0x9F,0x84,0x4D,0x73,0xA3,0xCA,0x9A,0x61,0x58,0x97,0xA3,0x27,0xFC,0x03,0x98,0x76,
        0x23,0x1D,0xC7,0x61,0x03,0x04,0xAE,0x56,0xBF,0x38,0x84,0x00,0x40,0xA7,0x0E,0xFD,
        0xFF,0x52,0xFE,0x03,0x6F,0x95,0x30,0xF1,0x97,0xFB,0xC0,0x85,0x60,0xD6,0x80,0x25,
        0xA9,0x63,0xBE,0x03,0x01,0x4E,0x38,0xE2,0xF9,0xA2,0x34,0xFF,0xBB,0x3E,0x03,0x44,
        0x78,0x00,0x90,0xCB,0x88,0x11,0x3A,0x94,0x65,0xC0,0x7C,0x63,0x87,0xF0,0x3C,0xAF,
        0xD6,0x25,0xE4,0x8B,0x38,0x0A,0xAC,0x72,0x21,0xD4,0xF8,0x07
    ];

const START_VECTOR: [u32; 8] = [0xe3a00301, 0xe5800208, 0xe3a00012, 0xe129f000, 
                                0xe59fd0b8, 0xe3a0001f, 0xe129f000, 0xe59fd0b0,
                            ];

// Not currently correct
// Loads SP from PC + 4, Adds 1 (for THUMB), BX to R0
const RESET_JUMP: [u32; 2] = [0xe28f0001, 0xe12fff10];

#[link_section = ".header.header"]
#[used]
static HEADER: Header = Header {
    start_code: 0xEA00002E,
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
    start_code2: 0xEA000008,
    boot_method: 0x00,
    slave_number: 0x00,
    reserved: [0u8; 26],
    //boot: START_VECTOR,
    //jump: RESET_JUMP,
};

#[link_section = ".header.reset_vector"]
#[used]
static RESET_VECTOR: unsafe extern "C" fn() -> ! = reset;

