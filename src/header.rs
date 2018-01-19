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
use pixel_format::PixelFormat;

#[derive(Debug, Clone)]
pub struct Header {
    // Size of this structure in bytes; set to 124
    // technically not required, we could take this out
    size: u32,

    // Flags indicating which members contain valid data
    flags: HeaderFlags,

    /// Surface height (in pixels)
    pub height: u32,

    /// Surface width (in pixels)
    pub width: u32,

    /// The pitch or number of bytes per scan line in an uncompressed texture;
    pub pitch: Option<u32>,

    /// The total number of bytes in a top level texture for a compressed texture
    pub linear_size: Option<u32>,

    /// Depth of a volume texture (in pixels)
    pub depth: Option<u32>,

    /// Number of mipmap levels
    pub mip_map_count: Option<u32>,

    // Unused (reserved)
    // technically not required, but we write back what we read
    reserved1: [u32; 11],

    /// The pixel format
    pub spf: PixelFormat,

    /// Specifies the complexity of the surfaces stored.
    pub caps: Caps,

    /// Additional detail about the surfaces stored
    pub caps2: Caps2,

    // Unused
    // technically not required, but we write back what we read
    caps3: u32,

    // Unused
    // technically not required, but we write back what we read
    caps4: u32,

    // Unused
    // technically not required, but we write back what we read
    reserved2: u32,
}

impl Header {
    pub fn new(height: u32, width: u32, pixel_format: PixelFormat) -> Header {
        Header {
            size: 124,
            flags: HeaderFlags::CAPS | HeaderFlags::HEIGHT | HeaderFlags::WIDTH
                | HeaderFlags::PIXELFORMAT,
            height: height,
            width: width,
            pitch: None,
            linear_size: None,
            depth: None,
            mip_map_count: None,
            reserved1: [0; 11],
            spf: pixel_format,
            caps: Caps::TEXTURE,
            caps2: Caps2::empty(),
            caps3: 0,
            caps4: 0,
            reserved2: 0,
        }
    }

    pub fn read<R: Read>(r: &mut R) -> Result<Header>
    {
        let size = r.read_u32::<LittleEndian>()?;
        if size != 124 {
            return Err(ErrorKind::InvalidField("Header struct size".to_owned()).into());
        }
        let flags = HeaderFlags::from_bits_truncate(
            r.read_u32::<LittleEndian>()?
        );
        let height = r.read_u32::<LittleEndian>()?;
        let width = r.read_u32::<LittleEndian>()?;
        let pitch_or_linear_size = r.read_u32::<LittleEndian>()?;
        let depth = r.read_u32::<LittleEndian>()?;
        let mip_map_count = r.read_u32::<LittleEndian>()?;
        let mut reserved1 = [0_u32; 11];
        r.read_u32_into::<LittleEndian>(&mut reserved1)?;
        let spf = PixelFormat::read(r)?;
        let caps = r.read_u32::<LittleEndian>()?;
        let caps2 = r.read_u32::<LittleEndian>()?;
        let caps3 = r.read_u32::<LittleEndian>()?;
        let caps4 = r.read_u32::<LittleEndian>()?;
        let reserved2 = r.read_u32::<LittleEndian>()?;
        Ok(Header {
            size: size,
            flags: flags,
            height: height,
            width: width,
            pitch: if flags.contains(HeaderFlags::PITCH) {
                Some(pitch_or_linear_size)
            } else {
                None
            },
            linear_size:  if flags.contains(HeaderFlags::LINEARSIZE) {
                Some(pitch_or_linear_size)
            } else {
                None
            },
            depth: if flags.contains(HeaderFlags::DEPTH) {
                Some(depth)
            } else {
                None
            },
            mip_map_count: if flags.contains(HeaderFlags::MIPMAPCOUNT) {
                Some(mip_map_count)
            } else {
                None
            },
            reserved1: reserved1,
            spf: spf,
            caps: Caps::from_bits_truncate(caps),
            caps2: Caps2::from_bits_truncate(caps2),
            caps3: caps3,
            caps4: caps4,
            reserved2: reserved2,
        })
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<()>
    {
        w.write_u32::<LittleEndian>(self.size)?;
        w.write_u32::<LittleEndian>(self.flags.bits())?;
        w.write_u32::<LittleEndian>(self.height)?;
        w.write_u32::<LittleEndian>(self.width)?;
        if let Some(pitch) = self.pitch {
            w.write_u32::<LittleEndian>(pitch)?;
        } else if let Some(ls) = self.linear_size {
            w.write_u32::<LittleEndian>(ls)?;
        } else {
            w.write_u32::<LittleEndian>(0)?;
        }
        w.write_u32::<LittleEndian>(self.depth.unwrap_or(0))?;
        w.write_u32::<LittleEndian>(self.mip_map_count.unwrap_or(0))?;
        for u in &self.reserved1 {
            w.write_u32::<LittleEndian>(*u)?;
        }
        self.spf.write(w)?;
        w.write_u32::<LittleEndian>(self.caps.bits())?;
        w.write_u32::<LittleEndian>(self.caps2.bits())?;
        w.write_u32::<LittleEndian>(self.caps3)?;
        w.write_u32::<LittleEndian>(self.caps4)?;
        w.write_u32::<LittleEndian>(self.reserved2)?;
        Ok(())
    }
}

bitflags! {
    pub struct HeaderFlags: u32 {
        /// Required in every DDS file
        const CAPS = 0x1;
        /// Required in every DDS file
        const HEIGHT = 0x2;
        /// Required in every DDS file
        const WIDTH = 0x4;
        /// Required when pitch is provided for an uncompressed texture
        const PITCH = 0x8;
        /// Required in every DDS file
        const PIXELFORMAT = 0x1000;
        /// Required in a mipmapped texture
        const MIPMAPCOUNT = 0x20000;
        /// Required when pitch is provided for a compressed texture
        const LINEARSIZE = 0x80000;
        /// Required in a depth texture
        const DEPTH = 0x800000;
    }
}

bitflags! {
    pub struct Caps: u32 {
        /// Optional; Must be used on any file that contains more than one surface
        /// (a mipmap, a cubic environment, or a mipmapped volume texture)
        const COMPLEX = 0x8;
        /// Optional; should be used for a mipmap
        const MIPMAP = 0x400000;
        /// Required
        const TEXTURE = 0x1000;
    }
}

bitflags! {
    pub struct Caps2: u32 {
        /// Required for a cube map
        const CUBEMAP = 0x200;
        /// Required when these surfaces are stored in a cubemap
        const CUBEMAP_POSITIVEX = 0x400;
        /// Required when these surfaces are stored in a cubemap
        const CUBEMAP_NEGATIVEX = 0x800;
        /// Required when these surfaces are stored in a cubemap
        const CUBEMAP_POSITIVEY = 0x1000;
        /// Required when these surfaces are stored in a cubemap
        const CUBEMAP_NEGATIVEY = 0x2000;
        /// Required when these surfaces are stored in a cubemap
        const CUBEMAP_POSITIVEZ = 0x4000;
        /// Required when these surfaces are stored in a cubemap
        const CUBEMAP_NEGATIVEZ = 0x8000;
        /// Required for a volume texture
        const VOLUME = 0x200000;
    }
}
