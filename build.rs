use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

extern crate gcc;

fn main() {
    // Put the linker scripts somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Copy linker script into output dir
    File::create(out.join("gba_cart.ld"))
        .unwrap()
        .write_all(include_bytes!("src/gba_cart.ld"))
        .unwrap();

     File::create(out.join("arm7tdmi.json"))
        .unwrap()
        .write_all(include_bytes!("arm7tdmi.json"))
        .unwrap();

    // Build gbafix (sorts out cart headers / checksums)
    Command::new("gcc").args(&["src/gbafix.c", "-o"])
                       .arg(&format!("{}/gbafix", out.to_str().unwrap()))
                       .status().unwrap();

    println!("cargo:rustc-link-search={}", out.display());
}
