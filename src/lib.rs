// The MIT License (MIT)
//
// Copyright (c) 2018 Michael Dilger
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#[macro_use]
extern crate bitflags;
extern crate byteorder;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate error_chain;

#[cfg(test)]
mod tests;

mod errors;
pub use errors::*;

mod pixel_format;
pub use pixel_format::{PixelFormat, PixelFormatFlags, FourCC};

mod format;
pub use format::{DxgiFormat, D3DFormat, DataFormat};

mod header;
pub use header::{Header, HeaderFlags, Caps, Caps2};

mod header10;
pub use header10::{Header10, D3D10ResourceDimension, MiscFlag, AlphaMode};

use std::io::{Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct Dds {
    // magic is implicit
    pub header: Header,
    pub header10: Option<Header10>,
    pub data: Vec<u8>,
}

impl Dds {
    const MAGIC: u32 = 0x20534444; // b"DDS " in little endian

    pub fn new(height: u32, width: u32, pixel_format: PixelFormat, data: Vec<u8>) -> Dds
    {
        Dds {
            header: Header::new(height, width, pixel_format),
            header10: None,
            data: data,
        }
    }

    pub fn read<R: Read>(r: &mut R) -> Result<Dds> {
        let magic = r.read_u32::<LittleEndian>()?;
        if magic != Self::MAGIC {
            return Err(ErrorKind::BadMagicNumber.into());
        }

        let header = Header::read(r)?;

        let header10 = if header.spf.fourcc == Some(FourCC(<FourCC>::DX10)) {
            Some(Header10::read(r)?)
        } else {
            None
        };

        let mut data: Vec<u8> = Vec::new();
        r.read_to_end(&mut data)?;
        Ok(Dds {
            header: header,
            header10: header10,
            data: data
        })
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<()> {
        w.write_u32::<LittleEndian>(Self::MAGIC)?;
        self.header.write(w)?;
        if let Some(ref header10) = self.header10 {
            header10.write(w)?;
        }
        w.write_all(&self.data)?;
        Ok(())
    }

    pub fn get_d3d_format(&self) -> Option<D3DFormat>
    {
        // FIXME: some d3d formats are equivalent to some dxgi formats.
        //    but we dont have a try_from() between them yet.
        //    Right now we will yield None if the format is dxgi, but
        //    later on we should try to convert.

        D3DFormat::try_from_pixel_format(&self.header.spf)
    }

    pub fn get_dxgi_format(&self) -> Option<DxgiFormat>
    {
        // FIXME: some d3d formats are equivalent to some dxgi formats.
        //    but we dont have a try_from() between them yet.
        //    Right now we will yield None if the format is d3d, but
        //    later on we should try to convert.
        if let Some(ref h10) = self.header10 {
            Some(h10.dxgi_format)
        } else {
            None
        }
    }

    pub fn get_main_texture_data_size(&self) -> Option<u32> {
        if let Some(pitch) = self.header.pitch {
            Some(pitch * self.header.height)
        }
        else if let Some(ls) = self.header.linear_size {
            Some(ls)
        }
        else {
            let format: Box<DataFormat> = if let Some(dxgi) = self.get_dxgi_format() {
                Box::new(dxgi)
            } else if let Some(d3d) = self.get_d3d_format() {
                Box::new(d3d)
            } else {
                return None
            };

            let pitch_height = format.get_pitch_height();
            let height = (self.header.height + (pitch_height-1))/ pitch_height;

            if let Some(pitch) = format.get_pitch(self.header.width) {
                Some(pitch * height)
            } else {
                None
            }
        }
    }
}
