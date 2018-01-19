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

use pixel_format::{PixelFormat, PixelFormatFlags, FourCC};

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum DxgiFormat {
        Unknown                     = 0,
        R32G32B32A32_Typeless       = 1,
        R32G32B32A32_Float          = 2,
        R32G32B32A32_UInt           = 3,
        R32G32B32A32_SInt           = 4,
        R32G32B32_Typeless          = 5,
        R32G32B32_Float             = 6,
        R32G32B32_UInt              = 7,
        R32G32B32_SInt              = 8,
        R16G16B16A16_Typeless       = 9,
        R16G16B16A16_Float          = 10,
        R16G16B16A16_UNorm          = 11,
        R16G16B16A16_UInt           = 12,
        R16G16B16A16_SNorm          = 13,
        R16G16B16A16_SInt           = 14,
        R32G32_Typeless             = 15,
        R32G32_Float                = 16,
        R32G32_UInt                 = 17,
        R32G32_SInt                 = 18,
        R32G8X24_Typeless           = 19,
        D32_Float_S8X24_UInt        = 20,
        R32_Float_X8X24_Typeless    = 21,
        X32_Typeless_G8X24_UInt     = 22,
        R10G10B10A2_Typeless        = 23,
        R10G10B10A2_UNorm           = 24,
        R10G10B10A2_UInt            = 25,
        R11G11B10_Float             = 26,
        R8G8B8A8_Typeless           = 27,
        R8G8B8A8_UNorm              = 28,
        R8G8B8A8_UNorm_sRGB         = 29,
        R8G8B8A8_UInt               = 30,
        R8G8B8A8_SNorm              = 31,
        R8G8B8A8_SInt               = 32,
        R16G16_Typeless             = 33,
        R16G16_Float                = 34,
        R16G16_UNorm                = 35,
        R16G16_UInt                 = 36,
        R16G16_SNorm                = 37,
        R16G16_SInt                 = 38,
        R32_Typeless                = 39,
        D32_Float                   = 40,
        R32_Float                   = 41,
        R32_UInt                    = 42,
        R32_SInt                    = 43,
        R24G8_Typeless              = 44,
        D24_UNorm_S8_UInt           = 45,
        R24_UNorm_X8_Typeless       = 46,
        X24_Typeless_G8_UInt        = 47,
        R8G8_Typeless               = 48,
        R8G8_UNorm                  = 49,
        R8G8_UInt                   = 50,
        R8G8_SNorm                  = 51,
        R8G8_SInt                   = 52,
        R16_Typeless                = 53,
        R16_Float                   = 54,
        D16_UNorm                   = 55,
        R16_UNorm                   = 56,
        R16_UInt                    = 57,
        R16_SNorm                   = 58,
        R16_SInt                    = 59,
        R8_Typeless                 = 60,
        R8_UNorm                    = 61,
        R8_UInt                     = 62,
        R8_SNorm                    = 63,
        R8_SInt                     = 64,
        A8_UNorm                    = 65,
        R1_UNorm                    = 66,
        R9G9B9E5_SharedExp          = 67,
        R8G8_B8G8_UNorm             = 68,
        G8R8_G8B8_UNorm             = 69,
        BC1_Typeless                = 70,
        BC1_UNorm                   = 71,
        BC1_UNorm_sRGB              = 72,
        BC2_Typeless                = 73,
        BC2_UNorm                   = 74,
        BC2_UNorm_sRGB              = 75,
        BC3_Typeless                = 76,
        BC3_UNorm                   = 77,
        BC3_UNorm_sRGB              = 78,
        BC4_Typeless                = 79,
        BC4_UNorm                   = 80,
        BC4_SNorm                   = 81,
        BC5_Typeless                = 82,
        BC5_UNorm                   = 83,
        BC5_SNorm                   = 84,
        B5G6R5_UNorm                = 85,
        B5G5R5A1_UNorm              = 86,
        B8G8R8A8_UNorm              = 87,
        B8G8R8X8_UNorm              = 88,
        R10G10B10_XR_Bias_A2_UNorm  = 89,
        B8G8R8A8_Typeless           = 90,
        B8G8R8A8_UNorm_sRGB         = 91,
        B8G8R8X8_Typeless           = 92,
        B8G8R8X8_UNorm_sRGB         = 93,
        BC6H_Typeless               = 94,
        BC6H_UF16                   = 95,
        BC6H_SF16                   = 96,
        BC7_Typeless                = 97,
        BC7_UNorm                   = 98,
        BC7_UNorm_sRGB              = 99,
        AYUV                        = 100,
        Y410                        = 101,
        Y416                        = 102,
        NV12                        = 103,
        P010                        = 104,
        P016                        = 105,
        Format_420_Opaque           = 106,
        YUY2                        = 107,
        Y210                        = 108,
        Y216                        = 109,
        NV11                        = 110,
        AI44                        = 111,
        IA44                        = 112,
        P8                          = 113,
        A8P8                        = 114,
        B4G4R4A4_UNorm              = 115,
        P208                        = 130,
        V208                        = 131,
        V408                        = 132,
        Force_UInt                  = -0x80000000, // 0xffffffff
    }
}

impl DxgiFormat {

    pub fn get_pitch(&self, width: u32) -> Option<u32>
    {
        // see https://msdn.microsoft.com/en-us/library/bb943991.aspx
        match *self {
            DxgiFormat::R8G8_B8G8_UNorm |
            DxgiFormat::G8R8_G8B8_UNorm => {
                return Some(((width+1)>>1) * 4);
            },
            _ => {}
        };

        if let Some(bpp) = self.get_bits_per_pixel() {
            Some((width * bpp as u32 + 7) / 8)
        }
        else if let Some(bs) = self.block_size() {
            Some(1.max(((width + 3)/4)) * bs)
        }
        else {
            None
        }
    }

    pub fn get_pitch_height(&self) -> u32
    {
        if self.block_size().is_some() {
            4
        } else {
            1
        }
    }

    pub fn get_bits_per_pixel(&self) -> Option<u32>
    {
        match *self {
            DxgiFormat::Unknown => None,

            DxgiFormat::R32G32B32A32_Typeless |
            DxgiFormat::R32G32B32A32_Float |
            DxgiFormat::R32G32B32A32_UInt |
            DxgiFormat::R32G32B32A32_SInt
                => Some(128),

            DxgiFormat::R32G32B32_Typeless |
            DxgiFormat::R32G32B32_Float |
            DxgiFormat::R32G32B32_UInt |
            DxgiFormat::R32G32B32_SInt
                => Some(96),

            DxgiFormat::R16G16B16A16_Typeless |
            DxgiFormat::R16G16B16A16_Float |
            DxgiFormat::R16G16B16A16_UNorm |
            DxgiFormat::R16G16B16A16_UInt |
            DxgiFormat::R16G16B16A16_SNorm |
            DxgiFormat::R16G16B16A16_SInt |
            DxgiFormat::R32G32_Typeless |
            DxgiFormat::R32G32_Float |
            DxgiFormat::R32G32_UInt |
            DxgiFormat::R32G32_SInt |
            DxgiFormat::R32G8X24_Typeless |
            DxgiFormat::D32_Float_S8X24_UInt |
            DxgiFormat::R32_Float_X8X24_Typeless |
            DxgiFormat::X32_Typeless_G8X24_UInt
                => Some(64),

            DxgiFormat::R10G10B10A2_Typeless |
            DxgiFormat::R10G10B10A2_UNorm |
            DxgiFormat::R10G10B10A2_UInt |
            DxgiFormat::R11G11B10_Float |
            DxgiFormat::R8G8B8A8_Typeless |
            DxgiFormat::R8G8B8A8_UNorm |
            DxgiFormat::R8G8B8A8_UNorm_sRGB |
            DxgiFormat::R8G8B8A8_UInt |
            DxgiFormat::R8G8B8A8_SNorm |
            DxgiFormat::R8G8B8A8_SInt |
            DxgiFormat::R16G16_Typeless |
            DxgiFormat::R16G16_Float |
            DxgiFormat::R16G16_UNorm |
            DxgiFormat::R16G16_UInt |
            DxgiFormat::R16G16_SNorm |
            DxgiFormat::R16G16_SInt |
            DxgiFormat::R32_Typeless |
            DxgiFormat::D32_Float |
            DxgiFormat::R32_Float |
            DxgiFormat::R32_UInt |
            DxgiFormat::R32_SInt |
            DxgiFormat::R24G8_Typeless |
            DxgiFormat::D24_UNorm_S8_UInt |
            DxgiFormat::R24_UNorm_X8_Typeless |
            DxgiFormat::X24_Typeless_G8_UInt
                => Some(32),

            DxgiFormat::R8G8_Typeless |
            DxgiFormat::R8G8_UNorm |
            DxgiFormat::R8G8_UInt |
            DxgiFormat::R8G8_SNorm |
            DxgiFormat::R8G8_SInt |
            DxgiFormat::R16_Typeless |
            DxgiFormat::R16_Float |
            DxgiFormat::D16_UNorm |
            DxgiFormat::R16_UNorm |
            DxgiFormat::R16_UInt |
            DxgiFormat::R16_SNorm |
            DxgiFormat::R16_SInt
                => Some(16),

            DxgiFormat::R8_Typeless |
            DxgiFormat::R8_UNorm |
            DxgiFormat::R8_UInt |
            DxgiFormat::R8_SNorm |
            DxgiFormat::R8_SInt |
            DxgiFormat::A8_UNorm
                => Some(8),

            DxgiFormat::R1_UNorm
                => Some(1),

            DxgiFormat::R9G9B9E5_SharedExp
                => Some(32),

            DxgiFormat::R8G8_B8G8_UNorm |
            DxgiFormat::G8R8_G8B8_UNorm
                => Some(16),

            DxgiFormat::B5G6R5_UNorm |
            DxgiFormat::B5G5R5A1_UNorm
                => Some(16),

            DxgiFormat::B8G8R8A8_UNorm |
            DxgiFormat::B8G8R8X8_UNorm |
            DxgiFormat::R10G10B10_XR_Bias_A2_UNorm |
            DxgiFormat::B8G8R8A8_Typeless |
            DxgiFormat::B8G8R8A8_UNorm_sRGB |
            DxgiFormat::B8G8R8X8_Typeless |
            DxgiFormat::B8G8R8X8_UNorm_sRGB
                => Some(32),

            DxgiFormat::AYUV => Some(32),
            DxgiFormat::Y410 => Some(10),
            DxgiFormat::Y416 => Some(16),
            DxgiFormat::NV12 => Some(12),
            DxgiFormat::P010 => Some(10),
            DxgiFormat::P016 => Some(16),
            DxgiFormat::Format_420_Opaque => Some(20),
            DxgiFormat::YUY2 => Some(16),
            DxgiFormat::Y210 => Some(10),
            DxgiFormat::Y216 => Some(16),
            DxgiFormat::NV11 => Some(11),
            DxgiFormat::AI44 => Some(44),
            DxgiFormat::IA44 => Some(44),
            DxgiFormat::P8 => Some(8),
            DxgiFormat::A8P8 => Some(16),
            DxgiFormat::B4G4R4A4_UNorm => Some(16),
            DxgiFormat::P208 => Some(8),
            DxgiFormat::V208 => Some(8),
            DxgiFormat::V408 => Some(8),

            _ => None,
        }
    }

    pub fn block_size(&self) -> Option<u32>
    {
        match *self {
            DxgiFormat::BC1_Typeless |
            DxgiFormat::BC1_UNorm |
            DxgiFormat::BC1_UNorm_sRGB
                => Some(8),

            DxgiFormat::BC2_Typeless |
            DxgiFormat::BC2_UNorm |
            DxgiFormat::BC2_UNorm_sRGB |
            DxgiFormat::BC3_Typeless |
            DxgiFormat::BC3_UNorm |
            DxgiFormat::BC3_UNorm_sRGB
                => Some(16),

            DxgiFormat::BC4_Typeless |
            DxgiFormat::BC4_UNorm |
            DxgiFormat::BC4_SNorm
                => Some(8),

            DxgiFormat::BC5_Typeless |
            DxgiFormat::BC5_UNorm |
            DxgiFormat::BC5_SNorm |
            DxgiFormat::BC6H_Typeless |
            DxgiFormat::BC6H_UF16 |
            DxgiFormat::BC6H_SF16 |
            DxgiFormat::BC7_Typeless |
            DxgiFormat::BC7_UNorm |
            DxgiFormat::BC7_UNorm_sRGB
                => Some(16),

            _ => None,
        }
    }
}

// We derive format from three possible sources:
//   PixelFormat
//   FourCC
//   DxgiFormat in header10

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

impl D3DFormat {
    pub fn get_pitch(&self, width: u32) -> Option<u32>
    {
        // see https://msdn.microsoft.com/en-us/library/bb943991.aspx
        match *self {
            D3DFormat::R8G8_B8G8 |
            D3DFormat::G8R8_G8B8 => {
                return Some(((width+1)>>1) * 4);
            },
            _ => {}
        };

        if let Some(bpp) = self.rgb_bit_count() {
            Some((width * bpp as u32 + 7) / 8)
        }
        else if let Some(bs) = self.block_size() {
            Some(1.max(((width + 3)/4)) * bs)
        }
        else {
            None
        }
    }

    pub fn get_pitch_height(&self) -> u32
    {
        if self.block_size().is_some() {
            4
        } else {
            1
        }
    }

    pub fn rgb_bit_count(&self) -> Option<u8> {
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

    pub fn block_size(&self) -> Option<u32> {
        match *self {
            D3DFormat::DXT1 => Some(8),
            D3DFormat::DXT2 |
            D3DFormat::DXT3 |
            D3DFormat::DXT4 |
            D3DFormat::DXT5
                => Some(16),
            _ => None,
        }
    }

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

    pub fn b_bit_mask(&self) -> Option<u32> {
        match *self {
            D3DFormat::A8B8G8R8 => Some(0x00ff_00000),
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

    pub fn try_from_pixel_format(pixel_format: &PixelFormat)
                                 -> Option<D3DFormat>
    {
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
                FourCC::DX10 => None,// should use try_from_header10
                _ => None,
            }
        }
        else {
            let rgb = pixel_format.flags.contains(PixelFormatFlags::RGB);
            let alpha = pixel_format.flags.contains(PixelFormatFlags::ALPHA) ||
                pixel_format.flags.contains(PixelFormatFlags::ALPHA_PIXELS);
            let lum = pixel_format.flags.contains(PixelFormatFlags::LUMINANCE);
            match (lum, rgb, alpha, pixel_format.rgb_bit_count,
                   pixel_format.r_bit_mask, pixel_format.g_bit_mask,
                   pixel_format.b_bit_mask, pixel_format.a_bit_mask)
            {
                (false,  true,  true, Some(32), Some(      0xff), Some(    0xff00), Some(  0xff0000), Some(0xff000000)) => Some(D3DFormat::A8B8G8R8),
                (false,  true, false, Some(32), Some(    0xffff), Some(0xffff0000), None,             None            ) => Some(D3DFormat::G16R16),
                (false,  true,  true, Some(32), Some(     0x3ff), Some(   0xffc00), Some(0x3ff00000), None            ) => Some(D3DFormat::A2B10G10R10),
                (false,  true,  true, Some(16), Some(    0x7c00), Some(     0x3e0), Some(      0x1f), Some(    0x8000)) => Some(D3DFormat::A1R5G5B5),
                (false,  true, false, Some(16), Some(    0xf800), Some(     0x7e0), Some(      0x1f), None            ) => Some(D3DFormat::R5G6B5),
                (false, false,  true, Some( 8), None,             None,             None,             Some(      0xff)) => Some(D3DFormat::A8),
                (false,  true,  true, Some(32), Some(  0xff0000), Some(    0xff00), Some(      0xff), Some(0xff000000)) => Some(D3DFormat::A8R8G8B8),
                (false,  true, false, Some(32), Some(  0xff0000), Some(    0xff00), Some(      0xff), None            ) => Some(D3DFormat::X8R8G8B8),
                (false,  true, false, Some(32), Some(      0xff), Some(    0xff00), Some(  0xff0000), None            ) => Some(D3DFormat::X8B8G8R8),
                (false,  true,  true, Some(32), Some(0x3ff00000), Some(   0xffc00), Some(     0x3ff), Some(0xc0000000)) => Some(D3DFormat::A2R10G10B10),
                (false,  true, false, Some(24), Some(  0xff0000), Some(    0xff00), Some(      0xff), None            ) => Some(D3DFormat::R8G8B8),
                (false,  true, false, Some(16), Some(    0x7c00), Some(     0x3e0), Some(      0x1f), None            ) => Some(D3DFormat::X1R5G5B5),
                (false,  true,  true, Some(16), Some(     0xf00), Some(      0xf0), Some(       0xf), Some(    0xf000)) => Some(D3DFormat::A4R4G4B4),
                (false,  true, false, Some(16), Some(     0xf00), Some(      0xf0), Some(       0xf), None            ) => Some(D3DFormat::X4R4G4B4),
                (false,  true,  true, Some(16), Some(      0xe0), Some(      0x1c), Some(       0x3), Some(    0xff00)) => Some(D3DFormat::A8R3G3B2),
                ( true, false,  true, Some(16), Some(      0xff), None,             None,             Some(    0xff00)) => Some(D3DFormat::A8L8),
                ( true, false, false, Some(16), Some(    0xffff), None,             None,             None            ) => Some(D3DFormat::L16),
                ( true, false, false, Some( 8), Some(      0xff), None,             None,             None            ) => Some(D3DFormat::L8),
                ( true, false,  true, Some( 8), Some(       0xf), None,             None,             Some(      0xf0)) => Some(D3DFormat::A4L4),
                _ => None
            }
        }
    }
}
