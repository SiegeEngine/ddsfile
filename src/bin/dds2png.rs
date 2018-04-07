
extern crate ddsfile;
extern crate png;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;

use png::{HasParameters, ColorType, BitDepth};
use ddsfile::*;

fn main() {
    let filename = match env::args().skip(1).next() {
        Some(arg) => arg,
        None => panic!("Usage: ddsinfo <filename>"),
    };

    let mut file = match File::open(&*filename) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let dds = match Dds::read(&mut file) {
        Ok(dds) => dds,
        Err(e) => panic!("{}", e),
    };

    let (colortype, bitdepth) = get_format(&dds);

    let outfilename = Path::new(&filename).with_extension("png");

    let outfile = File::create(outfilename).unwrap();
    let ref mut w = BufWriter::new(outfile);

    let mut encoder = png::Encoder::new(w, dds.get_width(), dds.get_height());
    encoder.set(colortype).set(bitdepth);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(dds.get_data(0).unwrap()).unwrap();
}

fn get_format(dds: &Dds) -> (ColorType, BitDepth)
{
    (ColorType::Grayscale, BitDepth::Eight)
/*    let format = dds.get_format().unwrap();

    // ColorType:  Grayscale, RGB, Indexed, GrayscaleAlpha, RGBA
    // BitDepth:  One, Two, Four, Eight, Sixteen

    let colortype = match dds.get_d3d_format() {
    }
*/
}
