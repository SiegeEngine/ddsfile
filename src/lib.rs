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

mod format;
pub use format::{DxgiFormat, D3DFormat, DataFormat, PixelFormat, PixelFormatFlags, FourCC};

mod header;
pub use header::{Header, HeaderFlags, Caps, Caps2};

mod header10;
pub use header10::{Header10, D3D10ResourceDimension, MiscFlag, AlphaMode};

use std::io::{Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// This is the main DirectDraw Surface file structure
pub struct Dds {
    // magic is implicit
    pub header: Header,
    pub header10: Option<Header10>,
    pub data: Vec<u8>,
}

impl Dds {
    const MAGIC: u32 = 0x20534444; // b"DDS " in little endian

    /// Create a new DirectDraw Surface.
    /// The data does NOT START OUT CONSISTENT at this point.  FIXME.
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

    pub fn get_format(&self) -> Option<Box<DataFormat>>
    {
        if let Some(dxgi) = self.get_dxgi_format() {
            Some(Box::new(dxgi))
        } else if let Some(d3d) = self.get_d3d_format() {
            Some(Box::new(d3d))
        } else {
            None
        }
    }

    fn get_main_texture_data_size(&self, format: &Box<DataFormat>) -> Option<u32> {
        if let Some(pitch) = self.header.pitch {
            Some(pitch * self.header.height)
        }
        else if let Some(ls) = self.header.linear_size {
            Some(ls)
        }
        else {
            let pitch_height = format.get_pitch_height();
            let height = (self.header.height + (pitch_height-1))/ pitch_height;

            if let Some(pitch) = format.get_pitch(self.header.width) {
                Some(pitch * height)
            } else {
                None
            }
        }
    }

    pub fn get_num_array_layers(&self) -> u32 {
        if let Some(ref h10) = self.header10 {
            h10.array_size
        } else {
            1 // just the 1 layer
        }
    }

    pub fn get_num_mipmap_levels(&self) -> u32 {
        if let Some(mmc) = self.header.mip_map_count {
            mmc
        } else {
            1 // just the main image
        }
    }

    /// This gets a reference to the data at the given `array_layer` (which should be
    /// 0 for textures with just one image).  If `mipmap_level` is specified, only
    /// that mipmap level will be included, otherwise all levels will be included.
    pub fn get_data<'a>(&'a self, array_layer: u32, mipmap_level: Option<u32>)
                        -> Option<&'a[u8]>
    {
        match self.get_offset_and_size(array_layer, mipmap_level)
        {
            Some((offset,size)) => return self.data.get(offset .. offset+size),
            None => None
        }
    }

    /// This gets a reference to the data at the given `array_layer` (which should be
    /// 0 for textures with just one image).  If `mipmap_level` is specified, only
    /// that mipmap level will be included, otherwise all levels will be included.
    pub fn get_mut_data<'a>(&'a mut self, array_layer: u32, mipmap_level: Option<u32>)
                            -> Option<&'a mut [u8]>
    {
        match self.get_offset_and_size(array_layer, mipmap_level)
        {
            Some((offset,size)) => return self.data.get_mut(offset .. offset+size),
            None => None
        }
    }

    fn get_offset_and_size(&self, array_layer: u32, mipmap_level: Option<u32>)
                           -> Option<(usize, usize)>
    {
        // Verify request bounds
        if array_layer >= self.get_num_array_layers() {
            return None;
        }
        if let Some(mml) = mipmap_level {
            if mml >= self.get_num_mipmap_levels() {
                return None;
            }
        };

        let format = match self.get_format() {
            Some(bx) => bx,
            None => return None,
        };

        let texture_size = match self.get_main_texture_data_size(&format) {
            Some(size) => size,
            None => return None
        };

        let min_mipmap_size = match format.get_minimum_mipmap_size_in_bytes() {
            Some(size) => size as usize,
            None => return None
        };

        let array_stride: usize = {
            let mut stride = 0;
            let mut current_mipsize = texture_size as usize;
            for _ in 0..self.get_num_mipmap_levels() {
                stride += current_mipsize;
                current_mipsize /= 4;
                if current_mipsize < min_mipmap_size {
                    current_mipsize = min_mipmap_size
                }
            }
            stride as usize
        };

        let mut offset = array_layer as usize * array_stride;

        match mipmap_level {
            None => {
                Some((offset, array_stride))
            },
            Some(mml) => {
                let mut current_mipsize: usize = texture_size as usize;
                for _ in 0..mml {
                    offset += current_mipsize;
                    current_mipsize /= 4;
                    if current_mipsize < min_mipmap_size {
                        current_mipsize = min_mipmap_size
                    }
                }
                Some((offset, current_mipsize))
            }
        }
    }
}
