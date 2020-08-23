# Rust GBA

Gameboy Advanced support for rust based on the [embedonomicon](https://japaric.github.io/embedonomicon/).

Check out `examples/gba-example-rainbow/` for a working example.

## Status

At the moment this pretty much just boots. In the future, it'd be neat to have drivers for the GBA peripherals included. If you implement anything, please do open a PR!

## Usage

1. Create a new `#[no_std]` binary project with `cargo new --bin`
2. Add `gba-core = { git = "https://github.com/ryankurte/rust-gba" }` as a dependency
3. Copy `arm7tdmi.json`,  `gba_cart.ld`, and `Makefile` from this repository into your project root
4. Edit `Makefile`, changing the `APP` variable to the name of your binary
5. Run `make clean all` to build the ROM image

Alternately, you can copy the example from `examples/gba-example-rainbow/` and work from there.
The `Makefile` in the example is pretty bare-bones, but it's enough to get a working ROM image.

## Helpers

This repo contains some (potentially) helpful utilities for GBA development. The source of these can be found in the `src/bin/` directory.

### gbafix

A Rust reimplementation of the `gbafix` tool, which you can use instead of the DevkitPro version when building GBA ROM images, is included as the `gbafix` binary.

## Resources

### Hardware
- [Wikipedia GBA Overview](https://en.wikipedia.org/wiki/Game_Boy_Advance)
- [GBA Hardware Overview](https://www.coranac.com/tonc/text/hardware.htm)

### Processor
- [Wikipedia ARM7TDMI](https://en.wikipedia.org/wiki/ARM7#ARM7TDMI)
- [ARM7TDMI Reference Manual](http://infocenter.arm.com/help/topic/com.arm.doc.ddi0210c/DDI0210B.pdf)

### Software
- [@exoticorn's gba-rust series](https://csclub.uwaterloo.ca/~tbelaire/blog/posts/gba-rust-1.html)
- [@exoticorn's gba-rust code](https://github.com/exoticorn/gba-rust)
- [DevKitPro gbafix rom fixer](https://github.com/devkitPro/gba-tools/blob/master/src/gbafix.c)
- [DevKitPro cartrige linker script](https://github.com/devkitPro/buildscripts/blob/master/dkarm-eabi/crtls/gba_cart.ld)
- [DevKitPro crt0 startup script](https://github.com/devkitPro/buildscripts/blob/master/dkarm-eabi/crtls/gba_crt0.s)
