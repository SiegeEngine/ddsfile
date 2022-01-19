
extern crate ddsfile;
use ddsfile::*;

use std::fs::File;
use std::env;

fn main() {
    let filename = match env::args().nth(1) {
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

    println!("{:?}", dds);
}
