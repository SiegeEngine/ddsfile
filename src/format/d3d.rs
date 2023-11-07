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

use super::pixel_format::{FourCC, PixelFormat, PixelFormatFlags};
use super::DataFormat;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum D3DFormat {
    A8B8G8R8,
    G16R16,
    A2B10G10R10,
    A1R5G5B5,
    R5G6B5,
    A8,
    A8R8G8B8,
    X8R8G8B8,
    X8B8G8R8,
    A2R10G10B10,
    R8G8B8,
    X1R5G5B5,
    A4R4G4B4,
    X4R4G4B4,
    A8R3G3B2,
    A8L8,
    L16,
    L8,
    A4L4,
    DXT1,
    DXT3,
    DXT5,
    R8G8_B8G8,
    G8R8_G8B8,
    A16B16G16R16,
    Q16W16V16U16,
    R16F,
    G16R16F,
    A16B16G16R16F,
    R32F,
    G32R32F,
    A32B32G32R32F,
    DXT2,
    DXT4,
    UYVY,
    YUY2,
    CXV8U8,
}

impl DataFormat for D3DFormat {
    fn get_pitch(&self, width: u32) -> Option<u32> {
        // see https://msdn.microsoft.com/en-us/library/bb943991.aspx
        match *self {
            D3DFormat::R8G8_B8G8 | D3DFormat::G8R8_G8B8 => {
                return Some(((width + 1) >> 1) * 4);
            }
            _ => {}
        };

        if let Some(bpp) = self.get_bits_per_pixel() {
            Some((width * bpp as u32 + 7) / 8)
        } else {
            self.get_block_size().map(|bs| 1.max((width + 3) / 4) * bs)
        }
    }

    fn get_bits_per_pixel(&self) -> Option<u8> {
        match *self {
            D3DFormat::A8B8G8R8 => Some(32),
            D3DFormat::G16R16 => Some(32),
            D3DFormat::A2B10G10R10 => Some(32),
            D3DFormat::A1R5G5B5 => Some(16),
            D3DFormat::R5G6B5 => Some(16),
            D3DFormat::A8 => Some(8),
            D3DFormat::A8R8G8B8 => Some(32),
            D3DFormat::X8R8G8B8 => Some(32),
            D3DFormat::X8B8G8R8 => Some(32),
            D3DFormat::A2R10G10B10 => Some(32),
            D3DFormat::R8G8B8 => Some(24),
            D3DFormat::X1R5G5B5 => Some(16),
            D3DFormat::A4R4G4B4 => Some(16),
            D3DFormat::X4R4G4B4 => Some(16),
            D3DFormat::A8R3G3B2 => Some(16),
            D3DFormat::A8L8 => Some(16),
            D3DFormat::L16 => Some(16),
            D3DFormat::L8 => Some(8),
            D3DFormat::A4L4 => Some(8),
            D3DFormat::DXT1 => None,
            D3DFormat::DXT3 => None,
            D3DFormat::DXT5 => None,
            D3DFormat::R8G8_B8G8 => Some(32),
            D3DFormat::G8R8_G8B8 => Some(32),
            D3DFormat::A16B16G16R16 => Some(64),
            D3DFormat::Q16W16V16U16 => Some(64),
            D3DFormat::R16F => Some(16),
            D3DFormat::G16R16F => Some(32),
            D3DFormat::A16B16G16R16F => Some(64),
            D3DFormat::R32F => Some(32),
            D3DFormat::G32R32F => Some(64),
            D3DFormat::A32B32G32R32F => Some(128),
            D3DFormat::DXT2 => None,
            D3DFormat::DXT4 => None,
            D3DFormat::UYVY => Some(16),
            D3DFormat::YUY2 => Some(16),
            D3DFormat::CXV8U8 => Some(16),
        }
    }

    fn get_block_size(&self) -> Option<u32> {
        match *self {
            D3DFormat::DXT1 => Some(8),
            D3DFormat::DXT2 | D3DFormat::DXT3 | D3DFormat::DXT4 | D3DFormat::DXT5 => Some(16),
            _ => None,
        }
    }

    fn get_fourcc(&self) -> Option<FourCC> {
        match *self {
            D3DFormat::A8B8G8R8 => None,
            D3DFormat::G16R16 => None,
            D3DFormat::A2B10G10R10 => None,
            D3DFormat::A1R5G5B5 => None,
            D3DFormat::R5G6B5 => None,
            D3DFormat::A8 => None,
            D3DFormat::A8R8G8B8 => None,
            D3DFormat::X8R8G8B8 => None,
            D3DFormat::X8B8G8R8 => None,
            D3DFormat::A2R10G10B10 => None,
            D3DFormat::R8G8B8 => None,
            D3DFormat::X1R5G5B5 => None,
            D3DFormat::A4R4G4B4 => None,
            D3DFormat::X4R4G4B4 => None,
            D3DFormat::A8R3G3B2 => None,
            D3DFormat::A8L8 => None,
            D3DFormat::L16 => None,
            D3DFormat::L8 => None,
            D3DFormat::A4L4 => None,
            D3DFormat::DXT1 => Some(FourCC(FourCC::DXT1)),
            D3DFormat::DXT3 => Some(FourCC(FourCC::DXT3)),
            D3DFormat::DXT5 => Some(FourCC(FourCC::DXT5)),
            D3DFormat::R8G8_B8G8 => Some(FourCC(FourCC::R8G8_B8G8)),
            D3DFormat::G8R8_G8B8 => Some(FourCC(FourCC::G8R8_G8B8)),
            D3DFormat::A16B16G16R16 => Some(FourCC(FourCC::A16B16G16R16)),
            D3DFormat::Q16W16V16U16 => Some(FourCC(FourCC::Q16W16V16U16)),
            D3DFormat::R16F => Some(FourCC(FourCC::R16F)),
            D3DFormat::G16R16F => Some(FourCC(FourCC::G16R16F)),
            D3DFormat::A16B16G16R16F => Some(FourCC(FourCC::A16B16G16R16F)),
            D3DFormat::R32F => Some(FourCC(FourCC::R32F)),
            D3DFormat::G32R32F => Some(FourCC(FourCC::G32R32F)),
            D3DFormat::A32B32G32R32F => Some(FourCC(FourCC::A32B32G32R32F)),
            D3DFormat::DXT2 => Some(FourCC(FourCC::DXT2)),
            D3DFormat::DXT4 => Some(FourCC(FourCC::DXT4)),
            D3DFormat::UYVY => Some(FourCC(FourCC::UYVY)),
            D3DFormat::YUY2 => Some(FourCC(FourCC::YUY2)),
            D3DFormat::CXV8U8 => Some(FourCC(FourCC::CXV8U8)),
        }
    }

    fn requires_extension(&self) -> bool {
        false
    }
}

impl D3DFormat {
    /// This gets the bitmask for the red channel pixels
    pub fn r_bit_mask(&self) -> Option<u32> {
        match *self {
            D3DFormat::A8B8G8R8 => Some(0x0000_00ff),
            D3DFormat::G16R16 => Some(0x0000_ffff),
            D3DFormat::A2B10G10R10 => Some(0x0000_03ff),
            D3DFormat::A1R5G5B5 => Some(0x7c00),
            D3DFormat::R5G6B5 => Some(0xf800),
            D3DFormat::A8 => None,
            D3DFormat::A8R8G8B8 => Some(0x00ff_0000),
            D3DFormat::X8R8G8B8 => Some(0x00ff_0000),
            D3DFormat::X8B8G8R8 => Some(0x0000_00ff),
            D3DFormat::A2R10G10B10 => Some(0x3ff0_0000),
            D3DFormat::R8G8B8 => Some(0xff_0000),
            D3DFormat::X1R5G5B5 => Some(0x7c00),
            D3DFormat::A4R4G4B4 => Some(0x0f00),
            D3DFormat::X4R4G4B4 => Some(0x0f00),
            D3DFormat::A8R3G3B2 => Some(0x00e0),
            D3DFormat::A8L8 => Some(0x00ff),
            D3DFormat::L16 => Some(0xffff),
            D3DFormat::L8 => Some(0xff),
            D3DFormat::A4L4 => Some(0x0f),
            _ => None,
        }
    }

    /// This gets the bitmask for the green channel pixels
    pub fn g_bit_mask(&self) -> Option<u32> {
        match *self {
            D3DFormat::A8B8G8R8 => Some(0x0000_ff00),
            D3DFormat::G16R16 => Some(0xffff_0000),
            D3DFormat::A2B10G10R10 => Some(0x000f_fc00),
            D3DFormat::A1R5G5B5 => Some(0x03e0),
            D3DFormat::R5G6B5 => Some(0x07e0),
            D3DFormat::A8 => None,
            D3DFormat::A8R8G8B8 => Some(0x0000_ff00),
            D3DFormat::X8R8G8B8 => Some(0x0000_ff00),
            D3DFormat::X8B8G8R8 => Some(0x0000_ff00),
            D3DFormat::A2R10G10B10 => Some(0x000f_fc00),
            D3DFormat::R8G8B8 => Some(0x00_ff00),
            D3DFormat::X1R5G5B5 => Some(0x03e0),
            D3DFormat::A4R4G4B4 => Some(0x00f0),
            D3DFormat::X4R4G4B4 => Some(0x00f0),
            D3DFormat::A8R3G3B2 => Some(0x001c),
            D3DFormat::A8L8 => None,
            D3DFormat::L16 => None,
            D3DFormat::L8 => None,
            D3DFormat::A4L4 => None,
            _ => None,
        }
    }

    /// This gets the bitmask for the blue channel pixels
    pub fn b_bit_mask(&self) -> Option<u32> {
        match *self {
            D3DFormat::A8B8G8R8 => Some(0x00ff_0000),
            D3DFormat::G16R16 => None,
            D3DFormat::A2B10G10R10 => Some(0x3ff0_0000),
            D3DFormat::A1R5G5B5 => Some(0x001f),
            D3DFormat::R5G6B5 => Some(0x001f),
            D3DFormat::A8 => None,
            D3DFormat::A8R8G8B8 => Some(0x0000_00ff),
            D3DFormat::X8R8G8B8 => Some(0x0000_00ff),
            D3DFormat::X8B8G8R8 => Some(0x00ff_0000),
            D3DFormat::A2R10G10B10 => Some(0x0000_03ff),
            D3DFormat::R8G8B8 => Some(0x00_00ff),
            D3DFormat::X1R5G5B5 => Some(0x001f),
            D3DFormat::A4R4G4B4 => Some(0x000f),
            D3DFormat::X4R4G4B4 => Some(0x000f),
            D3DFormat::A8R3G3B2 => Some(0x0003),
            D3DFormat::A8L8 => None,
            D3DFormat::L16 => None,
            D3DFormat::L8 => None,
            D3DFormat::A4L4 => None,
            _ => None,
        }
    }

    /// This gets the bitmask for the alpha channel pixels
    pub fn a_bit_mask(&self) -> Option<u32> {
        match *self {
            D3DFormat::A8B8G8R8 => Some(0xff00_0000),
            D3DFormat::G16R16 => None,
            D3DFormat::A2B10G10R10 => Some(0xc000_0000),
            D3DFormat::A1R5G5B5 => Some(0x8000),
            D3DFormat::R5G6B5 => None,
            D3DFormat::A8 => Some(0xff),
            D3DFormat::A8R8G8B8 => Some(0xff00_0000),
            D3DFormat::X8R8G8B8 => None,
            D3DFormat::X8B8G8R8 => None,
            D3DFormat::A2R10G10B10 => Some(0xc000_0000),
            D3DFormat::R8G8B8 => None,
            D3DFormat::X1R5G5B5 => None,
            D3DFormat::A4R4G4B4 => Some(0xf000),
            D3DFormat::X4R4G4B4 => None,
            D3DFormat::A8R3G3B2 => Some(0xff00),
            D3DFormat::A8L8 => Some(0xff00),
            D3DFormat::L16 => None,
            D3DFormat::L8 => None,
            D3DFormat::A4L4 => Some(0xf0),
            _ => None,
        }
    }

    /// This attempts to use `PixelFormat` data (e.g. from the dds.header.spf field)
    /// to determine the `D3DFormat`.
    pub fn try_from_pixel_format(pixel_format: &PixelFormat) -> Option<D3DFormat> {
        if let Some(ref fourcc) = pixel_format.fourcc {
            match fourcc.0 {
                FourCC::DXT1 => Some(D3DFormat::DXT1),
                FourCC::DXT2 => Some(D3DFormat::DXT2),
                FourCC::DXT3 => Some(D3DFormat::DXT3),
                FourCC::DXT4 => Some(D3DFormat::DXT4),
                FourCC::DXT5 => Some(D3DFormat::DXT5),
                FourCC::R8G8_B8G8 => Some(D3DFormat::R8G8_B8G8),
                FourCC::G8R8_G8B8 => Some(D3DFormat::G8R8_G8B8),
                FourCC::A16B16G16R16 => Some(D3DFormat::A16B16G16R16),
                FourCC::Q16W16V16U16 => Some(D3DFormat::Q16W16V16U16),
                FourCC::R16F => Some(D3DFormat::R16F),
                FourCC::G16R16F => Some(D3DFormat::G16R16F),
                FourCC::A16B16G16R16F => Some(D3DFormat::A16B16G16R16F),
                FourCC::R32F => Some(D3DFormat::R32F),
                FourCC::G32R32F => Some(D3DFormat::G32R32F),
                FourCC::A32B32G32R32F => Some(D3DFormat::A32B32G32R32F),
                FourCC::UYVY => Some(D3DFormat::UYVY),
                FourCC::YUY2 => Some(D3DFormat::YUY2),
                FourCC::CXV8U8 => Some(D3DFormat::CXV8U8),
                FourCC::DX10 => None, // should use try_from_header10
                _ => None,
            }
        } else {
            let rgb = pixel_format.flags.contains(PixelFormatFlags::RGB);
            let alpha = pixel_format.flags.contains(PixelFormatFlags::ALPHA)
                || pixel_format.flags.contains(PixelFormatFlags::ALPHA_PIXELS);
            let lum = pixel_format.flags.contains(PixelFormatFlags::LUMINANCE);
            #[rustfmt::skip]
            let format = match (
                lum,
                rgb,
                alpha,
                pixel_format.rgb_bit_count,
                pixel_format.r_bit_mask,
                pixel_format.g_bit_mask,
                pixel_format.b_bit_mask,
                pixel_format.a_bit_mask
            ) {
                // lum     rgb   alpha  rgb cnt                r bitmask         g bitmask         b bitmask         a bitmask
                (  false,  true,  true, Some(32),        Some(      0xff), Some(    0xff00), Some(  0xff0000), Some(0xff000000)) => Some(D3DFormat::A8B8G8R8),
                (  false,  true, false, Some(32),        Some(    0xffff), Some(0xffff0000), None,             None            ) => Some(D3DFormat::G16R16),
                (  false,  true,  true, Some(32),        Some(     0x3ff), Some(   0xffc00), Some(0x3ff00000), None            ) => Some(D3DFormat::A2B10G10R10),
                (  false,  true,  true, Some(16),        Some(    0x7c00), Some(     0x3e0), Some(      0x1f), Some(    0x8000)) => Some(D3DFormat::A1R5G5B5),
                (  false,  true, false, Some(16),        Some(    0xf800), Some(     0x7e0), Some(      0x1f), None            ) => Some(D3DFormat::R5G6B5),
                (  false, false,  true, Some( 8) | None, None,             None,             None,             Some(      0xff)) => Some(D3DFormat::A8),
                (  false,  true,  true, Some(32),        Some(  0xff0000), Some(    0xff00), Some(      0xff), Some(0xff000000)) => Some(D3DFormat::A8R8G8B8),
                (  false,  true, false, Some(32),        Some(  0xff0000), Some(    0xff00), Some(      0xff), None            ) => Some(D3DFormat::X8R8G8B8),
                (  false,  true, false, Some(32),        Some(      0xff), Some(    0xff00), Some(  0xff0000), None            ) => Some(D3DFormat::X8B8G8R8),
                (  false,  true,  true, Some(32),        Some(0x3ff00000), Some(   0xffc00), Some(     0x3ff), Some(0xc0000000)) => Some(D3DFormat::A2R10G10B10),
                (  false,  true, false, Some(24),        Some(  0xff0000), Some(    0xff00), Some(      0xff), None            ) => Some(D3DFormat::R8G8B8),
                (  false,  true, false, Some(16),        Some(    0x7c00), Some(     0x3e0), Some(      0x1f), None            ) => Some(D3DFormat::X1R5G5B5),
                (  false,  true,  true, Some(16),        Some(     0xf00), Some(      0xf0), Some(       0xf), Some(    0xf000)) => Some(D3DFormat::A4R4G4B4),
                (  false,  true, false, Some(16),        Some(     0xf00), Some(      0xf0), Some(       0xf), None            ) => Some(D3DFormat::X4R4G4B4),
                (  false,  true,  true, Some(16),        Some(      0xe0), Some(      0x1c), Some(       0x3), Some(    0xff00)) => Some(D3DFormat::A8R3G3B2),
                (   true, false,  true, Some(16),        Some(      0xff), None,             None,             Some(    0xff00)) => Some(D3DFormat::A8L8),
                (   true, false, false, Some(16),        Some(    0xffff), None,             None,             None            ) => Some(D3DFormat::L16),
                (   true, false, false, Some( 8),        Some(      0xff), None,             None,             None            ) => Some(D3DFormat::L8),
                (   true, false,  true, Some( 8),        Some(       0xf), None,             None,             Some(      0xf0)) => Some(D3DFormat::A4L4),
                _ => None
            };
            format
        }
    }
}
