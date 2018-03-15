// This is for re-tagging the format of a file that is tagged wrong.
// The data is not converted in any way.

extern crate ddsfile;

use ddsfile::*;

use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::env;

fn main() {
    let filename = match env::args().skip(1).next() {
        Some(arg) => arg,
        None => panic!("Usage: retag <ddsfile> <format>"),
    };

    let tag = match env::args().skip(2).next() {
        Some(arg) => arg,
        None => panic!("Usage: retag <ddsfile> <format>"),
    };
    // Rather than impl FromStr for dxgi and d3d formats, I'm hackily just adding
    // the ones I care about here.
    let format: DxgiFormat = match &*tag {
        "BC7_UNorm" => DxgiFormat::BC7_UNorm,
        "BC7_UNorm_sRGB" => DxgiFormat::BC7_UNorm_sRGB,
        _ => panic!("format not implemented")
    };

    let mut file = match OpenOptions::new().read(true).write(true).create(false)
        .open(&*filename)
    {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let mut dds = match Dds::read(&mut file) {
        Ok(dds) => dds,
        Err(e) => panic!("{}", e),
    };

    if let Some(ref mut h10) = dds.header10 {
        h10.dxgi_format = format;
    } else {
        panic!("d3d formats not implemented");
    }

    file.seek(SeekFrom::Start(0));

    if let Err(e) = dds.write(&mut file) {
        panic!("Error writing file: {:?}", e);
    }

    println!("Done.");
}
