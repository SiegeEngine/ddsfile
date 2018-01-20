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
use super::{D3DFormat, DxgiFormat, DataFormat};

#[derive(Debug, Clone)]
pub struct PixelFormat {
    /// Size of this structure in bytes; set to 32
    pub size: u32,

    /// Values which indicate what type of data is in the surface
    pub flags: PixelFormatFlags,

    /// Codes for specifying compressed or custom formats.
    pub fourcc: Option<FourCC>,

    /// Number of bits in an RGB (possibly including alpha) format. Valid when
    /// flags includes RGB or LUMINANCE.
    pub rgb_bit_count: Option<u32>,

    /// Red (or Y) mask for reading color data. For instance, given the A8R8G8B8 format,
    /// the red mask would be 0x00ff0000.
    pub r_bit_mask: Option<u32>,

    /// Green (or U) mask for reading color data. For instance, given the A8R8G8B8 format,
    /// the green mask would be 0x0000ff00.
    pub g_bit_mask: Option<u32>,

    /// Blue (or V) mask for reading color data. For instance, given the A8R8G8B8 format,
    /// the blue mask would be 0x000000ff
    pub b_bit_mask: Option<u32>,

    /// Alpha mask for reading alpha data. Valid of flags includes ALPHA_PIXELS or ALPHA.
    /// For instance, given the A8R8G8B8 format, the alpha mask would be 0xff000000
    pub a_bit_mask: Option<u32>,
}

impl PixelFormat {
    pub fn read<R: Read>(r: &mut R) -> Result<PixelFormat>
    {
        let size = r.read_u32::<LittleEndian>()?;
        if size != 32 {
            return Err(ErrorKind::InvalidField("Pixel format struct size".to_owned()).into());
        }
        let flags = PixelFormatFlags::from_bits_truncate(
            r.read_u32::<LittleEndian>()?
        );
        let fourcc = r.read_u32::<LittleEndian>()?;
        let rgb_bit_count = r.read_u32::<LittleEndian>()?;
        let r_bit_mask = r.read_u32::<LittleEndian>()?;
        let g_bit_mask = r.read_u32::<LittleEndian>()?;
        let b_bit_mask = r.read_u32::<LittleEndian>()?;
        let a_bit_mask = r.read_u32::<LittleEndian>()?;
        Ok(PixelFormat {
            size: size,
            flags: flags,
            fourcc: if flags.contains(PixelFormatFlags::FOURCC) {
                Some(FourCC(fourcc))
            } else {
                None
            },
            rgb_bit_count: if flags.contains(PixelFormatFlags::RGB)
                || flags.contains(PixelFormatFlags::LUMINANCE)
            {
                Some(rgb_bit_count)
            } else {
                None
            },
            r_bit_mask: if flags.contains(PixelFormatFlags::RGB) {
                Some(r_bit_mask)
            } else {
                None
            },
            g_bit_mask: if flags.contains(PixelFormatFlags::RGB) {
                Some(g_bit_mask)
            } else {
                None
            },
            b_bit_mask: if flags.contains(PixelFormatFlags::RGB) {
                Some(b_bit_mask)
            } else {
                None
            },
            a_bit_mask: if flags.contains(PixelFormatFlags::ALPHA_PIXELS)
                || flags.contains(PixelFormatFlags::ALPHA)
            {
                Some(a_bit_mask)
            } else {
                None
            }
        })
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<()>
    {
        w.write_u32::<LittleEndian>(self.size)?;
        w.write_u32::<LittleEndian>(self.flags.bits())?;
        w.write_u32::<LittleEndian>(self.fourcc.as_ref().unwrap_or(&FourCC(0)).0)?;
        w.write_u32::<LittleEndian>(self.rgb_bit_count.unwrap_or(0))?;
        w.write_u32::<LittleEndian>(self.r_bit_mask.unwrap_or(0))?;
        w.write_u32::<LittleEndian>(self.g_bit_mask.unwrap_or(0))?;
        w.write_u32::<LittleEndian>(self.b_bit_mask.unwrap_or(0))?;
        w.write_u32::<LittleEndian>(self.a_bit_mask.unwrap_or(0))?;
        Ok(())
    }
}

impl Default for PixelFormat {
    fn default() -> PixelFormat {
        PixelFormat {
            size: 32, // must be 32
            flags: PixelFormatFlags::empty(),
            fourcc: None,
            rgb_bit_count: None,
            r_bit_mask: None,
            g_bit_mask: None,
            b_bit_mask: None,
            a_bit_mask: None,
        }
    }
}

impl From<D3DFormat> for PixelFormat {
    fn from(format: D3DFormat) -> PixelFormat
    {
        let mut pf: PixelFormat = Default::default();
        if let Some(bpp) = format.get_bits_per_pixel() {
            pf.flags.insert(PixelFormatFlags::RGB);
            pf.rgb_bit_count = Some(bpp as u32)
        }
        else if let Some(fourcc) = format.get_fourcc() {
            pf.flags.insert(PixelFormatFlags::FOURCC);
            pf.fourcc = Some(fourcc);
        }
        if let Some(abitmask) = format.a_bit_mask() {
            pf.flags.insert(PixelFormatFlags::ALPHA_PIXELS);
            pf.a_bit_mask = Some(abitmask);
        }
        pf.r_bit_mask = format.r_bit_mask();
        pf.g_bit_mask = format.g_bit_mask();
        pf.b_bit_mask = format.b_bit_mask();
        pf
    }
}

impl From<DxgiFormat> for PixelFormat {
    fn from(format: DxgiFormat) -> PixelFormat
    {
        let mut pf: PixelFormat = Default::default();
        if let Some(bpp) = format.get_bits_per_pixel() {
            pf.flags.insert(PixelFormatFlags::RGB); // means uncompressed
            pf.rgb_bit_count = Some(bpp as u32)
        }
        pf.fourcc = Some(FourCC(FourCC::DX10)); // we always use extention for Dxgi
        pf.flags.insert(PixelFormatFlags::FOURCC);

        // flags::ALPHA_PIXELS is not set, use DX10 extension.
        // r_bit_mask, g_bit_mask, b_bit_mask and a_bit_mask are not set.
        // FIXME - we may need to set these in some circumstances.
        pf
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

// generate little-endian u32 from 4 bytes
// rust is not ready for this yet
/*
macro_rules! u32_code {
    ($w:expr) => {
        ((($w[0] as u32) <<  0) |
         (($w[1] as u32) <<  8) |
         (($w[2] as u32) << 16) |
         (($w[3] as u32) << 24) |
         ((*$w as [u8; 4])[0] as u32 * 0))
    }
}
 */

impl FourCC {
    pub const NONE: u32 = 0;

    // D3D formats
    pub const DXT1: u32 = 0x31545844; //u32_code!(b"DXT1");
    pub const DXT2: u32 = 0x32545844; //u32_code!(b"DXT2");
    pub const DXT3: u32 = 0x33545844; //u32_code!(b"DXT3");
    pub const DXT4: u32 = 0x34545844; //u32_code!(b"DXT4");
    pub const DXT5: u32 = 0x35545844; //u32_code!(b"DXT5");
    pub const R8G8_B8G8: u32 = 0x47424752; //u32_code!(b"RGBG");
    pub const G8R8_G8B8: u32 = 0x42475247;//u32_code!(b"GRGB");
    pub const A16B16G16R16: u32 = 36;
    pub const Q16W16V16U16: u32 = 110;
    pub const R16F: u32 = 111;
    pub const G16R16F: u32 = 112;
    pub const A16B16G16R16F: u32 = 113;
    pub const R32F: u32 = 114;
    pub const G32R32F: u32 = 115;
    pub const A32B32G32R32F: u32 = 116;
    pub const UYVY: u32 = 0x59565955; //u32_code!(b"UYVY");
    pub const YUY2: u32 = 0x32595559; //u32_code!(b"YUY2");
    pub const CXV8U8: u32 = 117;
    pub const DX10: u32 = 0x30315844; //u32_code!(b"DX10");

    // DXGI formats (different names, often for same things)
    pub const BC1_UNORM: u32 = 0x31545844; //u32_code!(b"DXT1");
    pub const BC2_UNORM: u32 = 0x33545844; //u32_code!(b"DXT3");
    pub const BC3_UNORM: u32 = 0x35545844; //u32_code!(b"DXT5");
    pub const BC4_UNORM: u32 = 0x55344342; //u32_code!(b"BC4U");
    pub const BC4_SNORM: u32 = 0x53344342; //u32_code!(b"BC4S");
    pub const BC5_UNORM: u32 = 0x32495441; //u32_code!(b"ATI2");
    pub const BC5_SNORM: u32 = 0x53354342; //u32_code!(b"BC5S");
    pub const R8G8_B8G8_UNORM: u32 = 0x47424752; //u32_code!(b"RGBG");
    pub const G8R8_G8B8_UNORM: u32 = 0x42475247; //u32_code!(b"GRGB");
    pub const R16G16B16A16_UNORM: u32 = 36;
    pub const R16G16B16A16_SNORM: u32 = 110;
    pub const R16_FLOAT: u32 = 111;
    pub const R16G16_FLOAT: u32 = 112;
    pub const R16G16B16A16_FLOAT: u32 = 113;
    pub const R32_FLOAT: u32 = 114;
    pub const R32G32_FLOAT: u32 = 115;
    pub const R32G32B32A32_FLOAT: u32 = 116;
}
