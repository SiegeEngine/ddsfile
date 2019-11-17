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

use crate::error::*;
use std::io::{Read, Write};
use std::fmt;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use enum_primitive::FromPrimitive;
use crate::format::DxgiFormat;

enum_from_primitive! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum D3D10ResourceDimension {
        Unknown = 0,
        Buffer = 1,
        Texture1D = 2,
        Texture2D = 3,
        Texture3D = 4,
    }
}

#[derive(Clone)]
pub struct Header10 {
    pub dxgi_format: DxgiFormat,
    pub resource_dimension: D3D10ResourceDimension,
    misc_flag: MiscFlag,
    pub array_size: u32,
    /// This is called misc_flags2 in the official documentation
    pub alpha_mode: AlphaMode,
}

impl fmt::Debug for Header10 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  Header10:")?;
        writeln!(f, "    dxgi_format: {:?}", self.dxgi_format)?;
        writeln!(f, "    resource_dimension: {:?}", self.resource_dimension)?;
        writeln!(f, "    misc_flag: {:?}", self.misc_flag)?;
        writeln!(f, "    array_size: {:?}", self.array_size)?;
        write!(f, "    alpha_mode: {:?}", self.alpha_mode)?;
        Ok(())
    }
}

impl Default for Header10 {
    fn default() -> Header10 {
        Header10 {
            dxgi_format: DxgiFormat::Unknown,
            resource_dimension: D3D10ResourceDimension::Unknown,
            misc_flag: MiscFlag::empty(),
            array_size: 0,
            alpha_mode: AlphaMode::Unknown,
        }
    }
}

impl Header10 {
    pub fn new(format: DxgiFormat, is_cubemap: bool,
               resource_dimension: D3D10ResourceDimension,
               array_size: u32,
               alpha_mode: AlphaMode)
               -> Header10
    {
        let mut flags = MiscFlag::empty();
        if is_cubemap {
            flags = flags | MiscFlag::TEXTURECUBE
        };
        Header10 {
            dxgi_format: format,
            resource_dimension: resource_dimension,
            misc_flag: flags,
            array_size: array_size,
            alpha_mode: alpha_mode,
        }
    }

    pub fn read<R: Read>(r: &mut R) -> Result<Header10, Error>
    {
        let dxgi_format = r.read_u32::<LittleEndian>()?;
        let resource_dimension = r.read_u32::<LittleEndian>()?;
        let misc_flag = MiscFlag::from_bits_truncate(
            r.read_u32::<LittleEndian>()?
        );
        let array_size = r.read_u32::<LittleEndian>()?;
        let alpha_mode = r.read_u32::<LittleEndian>()?;

        let dxgi_format_result: Result<DxgiFormat, Error> =
            DxgiFormat::from_u32(dxgi_format).ok_or(
                Error::InvalidField("dxgi_format".to_owned())
            );
        let resource_dimension_result: Result<D3D10ResourceDimension, Error> =
            D3D10ResourceDimension::from_u32(resource_dimension).ok_or(
                Error::InvalidField("resource_dimension".to_owned())
            );

        let alpha_mode: Result<AlphaMode, Error> =
            AlphaMode::from_u32(alpha_mode).ok_or(
                Error::InvalidField("alpha mode (misc_flags2)".to_owned())
            );

        Ok(Header10 {
            dxgi_format: dxgi_format_result?,
            resource_dimension: resource_dimension_result?,
            misc_flag: misc_flag,
            array_size: array_size,
            alpha_mode: alpha_mode?,
        })
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), Error>
    {
        w.write_u32::<LittleEndian>(self.dxgi_format as u32)?;
        w.write_u32::<LittleEndian>(self.resource_dimension as u32)?;
        w.write_u32::<LittleEndian>(self.misc_flag.bits())?;
        w.write_u32::<LittleEndian>(self.array_size)?;
        w.write_u32::<LittleEndian>(self.alpha_mode as u32)?;
        Ok(())
    }
}

bitflags! {
    pub struct MiscFlag: u32 {
        /// 2D Texture is a cube-map texture
        const TEXTURECUBE = 0x4;
    }
}

enum_from_primitive! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum AlphaMode {
        Unknown = 0x0,
        Straight = 0x1,
        PreMultiplied = 0x2,
        Opaque = 0x3,
        Custom = 0x4,
    }
}
