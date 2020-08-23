//! GBA ROM fixer.
//!
//! Currently does not accept any arguments beyond `--help`

#![feature(with_options)]
#![allow(unused_assignments)]

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::mem::size_of;
use std::path::Path;

extern crate gba_core;
use gba_core::header::Header;

/// Computes the header complement value for a given `Header`
fn header_complement(header: &Header) -> u8 {
    // Clone the header and set the checksum and complement values to zero
    let mut header = header.clone();
    header.checksum = 0;
    header.complement = 0;

    // Transform the header value into a byte slice
    let header_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts((&header as *const Header) as *const u8, size_of::<Header>())
    };

    // Iterate over the header value bytes and compute the complement
    let mut complement: u8 = 0;
    for n in 0xA0..0xBD {
        complement = complement.wrapping_add(header_bytes[n]);
    }

    // And finally, return the inverted complement value.
    // XXX: Not sure *why* we have to add one here, but it is required.
    !(0x19 + complement) + 1
}

/// Read the header from the file at `filename`, correct it's values, and
/// write it back out to the file.
///
/// The changes made to the header are as follows:
///
/// - Sets the logo to the correct value
/// - Sets the "fixed" field to the correct value
/// - Sets the device type field to the correct value
/// - Sets the checksum to zero
/// - Computes the header complement after the changes and sets the
///   complement field
fn process_file(filename: &str) -> std::io::Result<()> {
    // open file, seek to start
    let mut file = File::with_options()
        .read(true)
        .write(true)
        .open(Path::new(filename))?;
    file.seek(SeekFrom::Start(0))?;

    // read header bytes
    let mut header_buf = [0u8; size_of::<Header>()];
    file.read(&mut header_buf)?;

    // turn into a Header object
    let mut header: Header =
        unsafe { (*((header_buf.as_ptr() as *const u8) as *const Header)).clone() };

    // get known-good header
    let good_header = Header::default();

    // fix header fields
    header.logo = good_header.logo;
    header.fixed = good_header.fixed;
    header.device_type = good_header.device_type;
    header.checksum = 0; // checksum seems to be unused

    // calculate complement and store that
    header.complement = header_complement(&header);

    // turn Header object into bytes again
    let header_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts((&header as *const Header) as *const u8, size_of::<Header>())
    };

    // seek to start of file again, write header bytes
    file.seek(SeekFrom::Start(0))?;
    file.write_all(header_bytes)?;

    Ok(())
}

/// Processes command-line arguments, and calls [`process_file`] on each file
/// argument passed into the
fn main() {
    let mut executable = String::new();
    let mut captured = false;
    let mut files: Vec<String> = Vec::new();
    for (i, arg) in (0..).zip(std::env::args()) {
        // skip executable pathname
        if i == 0 {
            executable = arg;
            continue;
        }

        // determine when to stop processing arguments, outright assuming that
        // there will be no more arguments when we encounter an argument not
        // starting with a dash (or, if we get a double-dash)
        if arg == "--" {
            captured = true;
            continue;
        } else if arg.starts_with("-") {
            match arg.as_str() {
                "--help" => {
                    println!("usage: {} [options] <rom.gba> ...", executable);
                    println!("options:");
                    println!("\t--help\n\t\tdisplay this help message");
                    println!("");
                    return;
                }

                _ => {
                    eprintln!("error: unknown argument {:?}", arg);
                    eprintln!("See `{} --help` for options", executable);
                    return;
                }
            }
        } else {
            captured = true;
        }

        // push filenames to vec if we've stopped processing arguments
        if captured {
            files.push(arg);
        }
    }

    for file in files.iter() {
        match process_file(file) {
            Ok(_) => {
                println!("{:?} - success!", file);
            }

            Err(e) => {
                println!("{:?} - error: {}", file, e);
            }
        };
    }
}
