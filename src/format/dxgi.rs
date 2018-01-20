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

use super::pixel_format::{PixelFormat, PixelFormatFlags, FourCC};
use super::DataFormat;

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

impl DataFormat for DxgiFormat {
    fn get_pitch(&self, width: u32) -> Option<u32> {
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
        else if let Some(bs) = self.get_block_size() {
            Some(1.max(((width + 3)/4)) * bs)
        }
        else {
            None
        }
    }

    fn get_bits_per_pixel(&self) -> Option<u8> {
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

    fn get_block_size(&self) -> Option<u32> {
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

    fn get_fourcc(&self) -> Option<FourCC> {
        // note: we never use this. For Dxgi formats, we set FourCC to DX10 and
        // set the format in the header10 field. But these were the FourCCs that
        // were used prior to the header10 extension to DDS.
        match *self {
            DxgiFormat::BC1_UNorm => Some(FourCC(FourCC::BC1_UNORM)),
            DxgiFormat::BC2_UNorm => Some(FourCC(FourCC::BC2_UNORM)),
            DxgiFormat::BC3_UNorm => Some(FourCC(FourCC::BC3_UNORM)),
            DxgiFormat::BC4_UNorm => Some(FourCC(FourCC::BC4_UNORM)),
            DxgiFormat::BC4_SNorm => Some(FourCC(FourCC::BC4_SNORM)),
            DxgiFormat::BC5_UNorm => Some(FourCC(FourCC::BC5_UNORM)),
            DxgiFormat::BC5_SNorm  => Some(FourCC(FourCC::BC5_SNORM)),
            DxgiFormat::R8G8_B8G8_UNorm => Some(FourCC(FourCC::R8G8_B8G8_UNORM)),
            DxgiFormat::G8R8_G8B8_UNorm => Some(FourCC(FourCC::G8R8_G8B8_UNORM)),
            DxgiFormat::R16G16B16A16_UNorm => Some(FourCC(FourCC::R16G16B16A16_UNORM)),
            DxgiFormat::R16G16B16A16_SNorm => Some(FourCC(FourCC::R16G16B16A16_SNORM)),
            DxgiFormat::R16_Float => Some(FourCC(FourCC::R16_FLOAT)),
            DxgiFormat::R16G16_Float => Some(FourCC(FourCC::R16G16_FLOAT)),
            DxgiFormat::R16G16B16A16_Float => Some(FourCC(FourCC::R16G16B16A16_FLOAT)),
            DxgiFormat::R32_Float => Some(FourCC(FourCC::R32_FLOAT)),
            DxgiFormat::R32G32_Float => Some(FourCC(FourCC::R32G32_FLOAT)),
            DxgiFormat::R32G32B32A32_Float => Some(FourCC(FourCC::R32G32B32A32_FLOAT)),
            _ => None
        }
    }

    // sRGB, float, and compressed, and larger than u32, will all yield None.
    fn requires_extension(&self) -> bool {
        match *self {
            // Too big, and many are also not maskable types
            DxgiFormat::R32G32B32A32_Typeless |
            DxgiFormat::R32G32B32A32_Float |
            DxgiFormat::R32G32B32A32_UInt |
            DxgiFormat::R32G32B32A32_SInt |
            DxgiFormat::R32G32B32_Typeless |
            DxgiFormat::R32G32B32_Float |
            DxgiFormat::R32G32B32_UInt |
            DxgiFormat::R32G32B32_SInt |
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
                => true,

            // Not maskable types
            DxgiFormat::R10G10B10A2_Typeless |
            DxgiFormat::R11G11B10_Float |
            DxgiFormat::R8G8B8A8_Typeless |
            DxgiFormat::R8G8B8A8_UNorm_sRGB |
            DxgiFormat::R16G16_Typeless |
            DxgiFormat::R16G16_Float |
            DxgiFormat::R32_Typeless |
            DxgiFormat::D32_Float |
            DxgiFormat::R32_Float |
            DxgiFormat::R24G8_Typeless |
            DxgiFormat::R24_UNorm_X8_Typeless
                => true,

            // Not maskable types
            DxgiFormat::R8G8_Typeless |
            DxgiFormat::R16_Typeless |
            DxgiFormat::R16_Float
                => true,

            // Not maskable types
            DxgiFormat::R8_Typeless => true,

            // Not maskable types
            DxgiFormat::R9G9B9E5_SharedExp => true,

            // Not maskable types
            DxgiFormat::R10G10B10_XR_Bias_A2_UNorm |
            DxgiFormat::B8G8R8A8_Typeless |
            DxgiFormat::B8G8R8A8_UNorm_sRGB |
            DxgiFormat::B8G8R8X8_Typeless |
            DxgiFormat::B8G8R8X8_UNorm_sRGB
                => true,

            // Channels are not actual rgb
            DxgiFormat::AYUV |
            DxgiFormat::Y410 |
            DxgiFormat::Y416 |
            DxgiFormat::NV12 |
            DxgiFormat::P010 |
            DxgiFormat::P016 |
            DxgiFormat::Format_420_Opaque |
            DxgiFormat::YUY2 |
            DxgiFormat::Y210 |
            DxgiFormat::Y216 |
            DxgiFormat::NV11 |
            DxgiFormat::AI44 |
            DxgiFormat::IA44 |
            DxgiFormat::P8 |
            DxgiFormat::A8P8 |
            DxgiFormat::P208 |
            DxgiFormat::V208 |
            DxgiFormat::V408
                => true,

            _ => false
        }
    }
}
