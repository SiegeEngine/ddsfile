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
#[macro_use]
extern crate enum_primitive;

//#[cfg(test)]
//mod tests;

mod error;
pub use error::*;

mod format;
pub use format::{DxgiFormat, D3DFormat, DataFormat, PixelFormat, PixelFormatFlags, FourCC};

mod header;
pub use header::{Header, HeaderFlags, Caps, Caps2};

mod header10;
pub use header10::{Header10, D3D10ResourceDimension, MiscFlag, AlphaMode};

use std::io::{Read, Write};
use std::fmt;
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
                   mipmap_levels: Option<u32>, caps2: Option<Caps2>) -> Result<Dds, Error>
    {
        let size = match get_texture_size(format.get_pitch(width), None,
                                          format.get_pitch_height(),
                                          height, depth)
        {
            Some(s) => s,
            None => return Err(Error::UnsupportedFormat),
        };

        let mml = mipmap_levels.unwrap_or(1);
        let min_mipmap_size = match format.get_minimum_mipmap_size_in_bytes() {
            Some(mms) => mms,
            None => return Err(Error::UnsupportedFormat),
        };
        let array_stride = get_array_stride(size, min_mipmap_size, mml);

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
                    -> Result<Dds, Error>
    {
        let arraysize = match array_layers {
            Some(s) => s,
            None => 1,
        };

        let size = match get_texture_size(format.get_pitch(width), None,
                                          format.get_pitch_height(),
                                          height, depth)
        {
            Some(s) => s,
            None => return Err(Error::UnsupportedFormat),
        };

        let mml = mipmap_levels.unwrap_or(1);
        let min_mipmap_size = match format.get_minimum_mipmap_size_in_bytes() {
            Some(mms) => mms,
            None => return Err(Error::UnsupportedFormat),
        };
        let array_stride = get_array_stride(size, min_mipmap_size, mml);

        let data_size = arraysize * array_stride;

        let arraysize = if is_cubemap { arraysize / 6 }  else { arraysize };
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
    pub fn read<R: Read>(mut r: R) -> Result<Dds, Error> {
        let magic = r.read_u32::<LittleEndian>()?;
        if magic != Self::MAGIC {
            return Err(Error::BadMagicNumber);
        }

        let header = Header::read(&mut r)?;

        let header10 = if header.spf.fourcc == Some(FourCC(<FourCC>::DX10)) {
            Some(Header10::read(&mut r)?)
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
    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), Error> {
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
            DxgiFormat::try_from_pixel_format(&self.header.spf)
        }
    }

    /// Get the format of the DDS as a trait (type-erasure)
    pub fn get_format(&self) -> Option<Box<dyn DataFormat>>
    {
        if let Some(dxgi) = self.get_dxgi_format() {
            Some(Box::new(dxgi))
        } else if let Some(d3d) = self.get_d3d_format() {
            Some(Box::new(d3d))
        } else {
            None
        }
    }

    pub fn get_width(&self) -> u32 {
        self.header.width
    }

    pub fn get_height(&self) -> u32 {
        self.header.height
    }

    pub fn get_depth(&self) -> u32 {
        self.header.depth.unwrap_or(1)
    }

    pub fn get_bits_per_pixel(&self) -> Option<u32> {
        // Try format first
        if let Some(format) = self.get_format() {
            if let Some(bpp) = format.get_bits_per_pixel() {
                return Some(bpp as u32);
            }
        }
        // Fall back to pixel_format rgb_bit_count field
        if let Some(bpp) = self.header.spf.rgb_bit_count {
            return Some(bpp);
        }
        None
    }

    pub fn get_pitch(&self) -> Option<u32> {
        // Try format first
        if let Some(format) = self.get_format() {
            if let Some(pitch) = format.get_pitch(self.header.width) {
                return Some(pitch);
            }
        }
        // Then try header.pitch
        if let Some(pitch) = self.header.pitch {
            return Some(pitch);
        }

        // Then try to calculate it ourselves
        if let Some(bpp) = self.get_bits_per_pixel() {
            return Some((bpp * self.get_width() + 7) / 8)
        }
        None
    }

    pub fn get_pitch_height(&self) -> u32 {
        if let Some(format) = self.get_format() {
            format.get_pitch_height()
        } else {
            1
        }
    }

    pub fn get_main_texture_size(&self) -> Option<u32> {
        get_texture_size(
            self.get_pitch(), self.header.linear_size,
            self.get_pitch_height(),
            self.header.height, self.header.depth)
    }

    pub fn get_array_stride(&self) -> Result<u32, Error>
    {
        let size = match self.get_main_texture_size() {
            Some(s) => s,
            None => return Err(Error::UnsupportedFormat),
        };
        let mml = self.get_num_mipmap_levels();
        let min_mipmap_size = self.get_min_mipmap_size_in_bytes();
        Ok(get_array_stride(size, min_mipmap_size, mml))
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

    pub fn get_min_mipmap_size_in_bytes(&self) -> u32 {
        if let Some(format) = self.get_format() {
            if let Some(min) = format.get_minimum_mipmap_size_in_bytes() {
                return min;
            }
        }
        if let Some(bpp) = self.get_bits_per_pixel() {
            (bpp + 7) / 8
        } else {
            1
        }
    }

    /// This gets a reference to the data at the given `array_layer` (which should be
    /// 0 for textures with just one image).
    pub fn get_data<'a>(&'a self, array_layer: u32)
                        -> Result<&'a[u8], Error>
    {
        let (offset,size) = self.get_offset_and_size(array_layer)?;
        let offset = offset as usize;
        let size = size as usize;
        self.data.get(offset .. offset+size).ok_or(
            Error::OutOfBounds)
    }

    /// This gets a reference to the data at the given `array_layer` (which should be
    /// 0 for textures with just one image).
    pub fn get_mut_data<'a>(&'a mut self, array_layer: u32)
                            -> Result<&'a mut [u8], Error>
    {
        let (offset,size) = self.get_offset_and_size(array_layer)?;
        let offset = offset as usize;
        let size = size as usize;
        self.data.get_mut(offset .. offset+size).ok_or(
            Error::OutOfBounds)
    }

    fn get_offset_and_size(&self, array_layer: u32) -> Result<(u32, u32), Error>
    {
        // Verify request bounds
        if array_layer >= self.get_num_array_layers() {
            return Err(Error::OutOfBounds);
        }
        let array_stride = self.get_array_stride()?;
        let offset = array_layer * array_stride;

        Ok((offset, array_stride))
    }
}

fn get_texture_size(pitch: Option<u32>, linear_size: Option<u32>, pitch_height: u32,
                    height: u32, depth: Option<u32>)
                    -> Option<u32>
{
    let depth = depth.unwrap_or(1);

    if let Some(ls) = linear_size {
        Some(ls)
    }
    else if let Some(pitch) = pitch {
        let row_height = (height + (pitch_height-1))/ pitch_height;
        Some(pitch * row_height * depth)
    }
    else {
        None
    }
}

fn get_array_stride(texture_size: u32,
                    min_mipmap_size: u32,
                    mipmap_levels: u32)
                    -> u32
{
    let mut stride: u32 = 0;
    let mut current_mipsize: u32 = texture_size;
    for _ in 0..mipmap_levels {
        stride += current_mipsize;
        current_mipsize /= 4;
        if current_mipsize < min_mipmap_size {
            current_mipsize = min_mipmap_size;
        }
    }
    stride
}

impl fmt::Debug for Dds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Dds:")?;
        if let Some(d3dformat) = self.get_d3d_format() {
            writeln!(f, "  Format: {:?}", d3dformat)?;
        } else if let Some(dxgiformat) = self.get_dxgi_format() {
            writeln!(f, "  Format: {:?}", dxgiformat)?;
        } else if let Some(ref fourcc) = self.header.spf.fourcc {
            writeln!(f, "  Format: FOURCC={:?} (Unknown)", fourcc)?;
        } else {
            writeln!(f, "  Format UNSPECIFIED")?;
        }
        write!(f, "{:?}", self.header)?;
        if let Some(ref h10) = self.header10 {
            write!(f, "{:?}", h10)?;
        }
        writeln!(f, "  (data elided)")?;
        Ok(())
    }
}
