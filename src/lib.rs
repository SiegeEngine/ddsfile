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

//! The main entry point for this library is the `Dds` type.

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

    /// Create a new DirectDraw Surface with a D3DFormat
    pub fn new_d3d(height: u32, width: u32, depth: Option<u32>, format: D3DFormat,
                   mipmap_levels: Option<u32>, caps2: Option<Caps2>) -> Result<Dds>
    {
        let boxformat: Box<DataFormat> = Box::new(format);

        let size = match get_texture_size(None, None, &boxformat,
                                          width, height, depth)
        {
            Some(s) => s,
            None => return Err(ErrorKind::UnsupportedFormat.into()),
        };

        let mml = mipmap_levels.unwrap_or(1);
        let array_stride = get_array_stride(size, &boxformat, mml)?;

        let data_size = array_stride;

        Ok(Dds {
            header: Header::new_d3d(height, width, depth, format, mipmap_levels,
                                    caps2)?,
            header10: None,
            data: vec![0; data_size as usize],
        })
    }

    /// Create a new DirectDraw Surface with a DxgiFormat
    pub fn new_dxgi(height: u32, width: u32, depth: Option<u32>,
                    format: DxgiFormat,
                    mipmap_levels: Option<u32>, array_layers: Option<u32>,
                    caps2: Option<Caps2>, is_cubemap: bool,
                    resource_dimension: D3D10ResourceDimension,
                    alpha_mode: AlphaMode)
                    -> Result<Dds>
    {
        let arraysize = match array_layers {
            Some(s) => s,
            None => 1,
        };
        let boxformat: Box<DataFormat> = Box::new(format);

        let size = match get_texture_size(None, None, &boxformat,
                                          width, height,
                                          depth)
        {
            Some(s) => s,
            None => return Err(ErrorKind::UnsupportedFormat.into()),
        };

        let mml = mipmap_levels.unwrap_or(1);
        let array_stride = get_array_stride(size, &boxformat, mml)?;

        let data_size = arraysize * array_stride;

        let header10 = Header10::new(
            format, is_cubemap, resource_dimension,
            arraysize, alpha_mode);

        Ok(Dds {
            header: Header::new_dxgi(height, width, depth, format, mipmap_levels,
                                     array_layers, caps2)?,
            header10: Some(header10),
            data: vec![0; data_size as usize],
        })
    }

    /// Read a DDS file
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

    /// Write to a DDS file
    pub fn write<W: Write>(&self, w: &mut W) -> Result<()> {
        w.write_u32::<LittleEndian>(Self::MAGIC)?;
        self.header.write(w)?;
        if let Some(ref header10) = self.header10 {
            header10.write(w)?;
        }
        w.write_all(&self.data)?;
        Ok(())
    }

    /// Attempt to get the format of this DDS, presuming it is a D3DFormat.
    pub fn get_d3d_format(&self) -> Option<D3DFormat>
    {
        // FIXME: some d3d formats are equivalent to some dxgi formats.
        //    but we dont have a try_from() between them yet.
        //    Right now we will yield None if the format is dxgi, but
        //    later on we should try to convert.

        D3DFormat::try_from_pixel_format(&self.header.spf)
    }

    /// Attempt to get the format of this DDS, presuming it is a DxgiFormat.
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

    /// Get the format of the DDS as a trait (type-erasure)
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
    /// 0 for textures with just one image).
    pub fn get_data<'a>(&'a self, array_layer: u32)
                        -> Result<&'a[u8]>
    {
        let (offset,size) = self.get_offset_and_size(array_layer)?;
        let offset = offset as usize;
        let size = size as usize;
        self.data.get(offset .. offset+size).ok_or(
            ErrorKind::OutOfBounds.into())
    }

    /// This gets a reference to the data at the given `array_layer` (which should be
    /// 0 for textures with just one image).
    pub fn get_mut_data<'a>(&'a mut self, array_layer: u32)
                            -> Result<&'a mut [u8]>
    {
        let (offset,size) = self.get_offset_and_size(array_layer)?;
        let offset = offset as usize;
        let size = size as usize;
        self.data.get_mut(offset .. offset+size).ok_or(
            ErrorKind::OutOfBounds.into())
    }

    fn get_offset_and_size(&self, array_layer: u32) -> Result<(u32, u32)>
    {
        // Verify request bounds
        if array_layer >= self.get_num_array_layers() {
            return Err(ErrorKind::OutOfBounds.into());
        }

        let format = match self.get_format() {
            Some(bx) => bx,
            None => return Err(ErrorKind::UnsupportedFormat.into())
        };

        let texture_size: u32 = match get_texture_size(
            self.header.pitch, self.header.linear_size, &format,
            self.header.width, self.header.height,
            self.header.depth
        ) {
            Some(size) => size,
            None => return Err(ErrorKind::UnsupportedFormat.into()),
        };

        let array_stride = get_array_stride(
            texture_size, &format, self.get_num_mipmap_levels())?;

        let offset = array_layer * array_stride;

        Ok((offset, array_stride))
    }
}

fn get_texture_size(pitch: Option<u32>, linear_size: Option<u32>,
                    format: &Box<DataFormat>,
                    width: u32, height: u32, depth: Option<u32>)
                    -> Option<u32>
{
    let depth = depth.unwrap_or(1);
    if let Some(pitch) = pitch {
        Some(pitch * height * depth)
    }
    else if let Some(ls) = linear_size {
        Some(ls * depth)
    }
    else {
        let pitch_height = format.get_pitch_height();
        let row_height = (height + (pitch_height-1))/ pitch_height;
        if let Some(pitch) = format.get_pitch(width) {
            Some(pitch * row_height * depth)
        } else {
            None
        }
    }
}

fn get_array_stride(texture_size: u32,
                    format: &Box<DataFormat>,
                    mipmap_levels: u32)
                    -> Result<u32>
{
    let min_mipmap_size = match format.get_minimum_mipmap_size_in_bytes() {
        Some(size) => size,
        None => return Err(ErrorKind::UnsupportedFormat.into()),
    };

    let mut stride: u32 = 0;
    let mut current_mipsize: u32 = texture_size;
    for _ in 0..mipmap_levels {
        stride += current_mipsize;
        current_mipsize /= 4;
        if current_mipsize < min_mipmap_size {
            current_mipsize = min_mipmap_size;
        }
    }
    Ok(stride)
}
