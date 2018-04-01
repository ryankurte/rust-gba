# Rust GBA

Gameboy Advanced support for rust based on the [embedonomicon](https://japaric.github.io/embedonomicon/).

Check out [ryankurte/rust-gba-example](https://github.com/ryankurte/rust-gba-example) for a working example.

## Status

At the moment this pretty much just boots. In the future, it'd be neat to have drivers for the GBA peripherals included. If you implement anything, please do open a PR!

## Usage

1. Create a new `#[no_std]` binary project with `cargo new --bin`
2. Add `gba = { git = "https://github.com/ryankurte/rust-gba" }` as a dependency
3. Copy `arm7tdmi.json`, `.cargo/config`, `Xargo.toml` and optionally `makefile` from this into your project root
4. Build with `xargo -v rustc -- -C link-arg=-emain --verbose`

For more useful commands, check out the [makefile](https://github.com/ryankurte/rust-gba/blob/master/makefile)


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




