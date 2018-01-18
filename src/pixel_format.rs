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

use errors::*;
use std::io::{Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug, Clone)]
pub struct PixelFormat {
    /// Size of this structure in bytes; set to 32
    pub size: u32,

    /// Values which indicate what type of data is in the surface
    pub flags: PixelFormatFlags,

    /// Codes for specifying compressed or custom formats.
    pub fourcc: FourCC,

    /// Number of bits in an RGB (possibly including alpha) format. Valid when
    /// flags includes RGB or LUMINANCE.
    pub rgb_bit_count: u32,

    /// Red (or Y) mask for reading color data. For instance, given the A8R8G8B8 format,
    /// the red mask would be 0x00ff0000.
    pub r_bit_mask: u32,

    /// Green (or U) mask for reading color data. For instance, given the A8R8G8B8 format,
    /// the green mask would be 0x0000ff00.
    pub g_bit_mask: u32,

    /// Blue (or V) mask for reading color data. For instance, given the A8R8G8B8 format,
    /// the blue mask would be 0x000000ff
    pub b_bit_mask: u32,

    /// Alpha mask for reading alpha data. Valid of flags includes ALPHA_PIXELS or ALPHA.
    /// For instance, given the A8R8G8B8 format, the alpha mask would be 0xff000000
    pub a_bit_mask: u32,
}

impl PixelFormat {
    pub fn read<R: Read>(r: &mut R) -> Result<PixelFormat>
    {
        let size = r.read_u32::<LittleEndian>()?;
        if size != 32 {
            return Err(ErrorKind::InvalidField("Pixel format struct size".to_owned()).into());
        }
        let flags = r.read_u32::<LittleEndian>()?;
        let fourcc = r.read_u32::<LittleEndian>()?;
        let rgb_bit_count = r.read_u32::<LittleEndian>()?;
        let r_bit_mask = r.read_u32::<LittleEndian>()?;
        let g_bit_mask = r.read_u32::<LittleEndian>()?;
        let b_bit_mask = r.read_u32::<LittleEndian>()?;
        let a_bit_mask = r.read_u32::<LittleEndian>()?;
        Ok(PixelFormat {
            size: size,
            flags: PixelFormatFlags::from_bits_truncate(flags),
            fourcc: FourCC(fourcc),
            rgb_bit_count: rgb_bit_count,
            r_bit_mask: r_bit_mask,
            g_bit_mask: g_bit_mask,
            b_bit_mask: b_bit_mask,
            a_bit_mask: a_bit_mask,
        })
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<()>
    {
        w.write_u32::<LittleEndian>(self.size)?;
        w.write_u32::<LittleEndian>(self.flags.bits())?;
        w.write_u32::<LittleEndian>(self.fourcc.0)?;
        w.write_u32::<LittleEndian>(self.rgb_bit_count)?;
        w.write_u32::<LittleEndian>(self.r_bit_mask)?;
        w.write_u32::<LittleEndian>(self.g_bit_mask)?;
        w.write_u32::<LittleEndian>(self.b_bit_mask)?;
        w.write_u32::<LittleEndian>(self.a_bit_mask)?;
        Ok(())
    }
}

bitflags! {
    pub struct PixelFormatFlags: u32 {
        /// Texture contains alpha data.
        const ALPHA_PIXELS = 0x1;
        /// Alpha channel only uncomressed data (used in older DDS files)
        const ALPHA = 0x2;
        /// Texture contains compressed RGB data.
        const FOURCC = 0x4;
        /// Texture contains uncompressed RGB data.
        const RGB = 0x40;
        /// YUV uncompressed data (used in older DDS files)
        const YUV = 0x200;
        /// Single channel color uncompressed data (used in older DDS files)
        const LUMINANCE = 0x20000;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FourCC(pub u32);

impl FourCC {
    pub const NONE: u32 = 0;
    pub const DXT1: u32 = 0x31545844; // "DXT1"
    pub const DXT2: u32 = 0x32545844; // b"DXT2"
    pub const DXT3: u32 = 0x33545844; // b"DXT3"
    pub const DXT4: u32 = 0x34545844; // b"DXT4"
    pub const DXT5: u32 = 0x35545844; // b"DXT5"
    pub const DX10: u32 = 0x30315844; // b"DX10"
}
