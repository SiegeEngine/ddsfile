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

mod pixel_format;
pub use self::pixel_format::{PixelFormat, PixelFormatFlags, FourCC};

mod d3d;
pub use self::d3d::D3DFormat;

mod dxgi;
pub use self::dxgi::DxgiFormat;

pub trait DataFormat {
    /// This gets the number of bytes required to store one row of data
    fn get_pitch(&self, width: u32) -> Option<u32>;

    /// This gets the height of each row of data. Normally it is 1, but for block
    /// compressed textures, each row is 4 pixels high.
    fn get_pitch_height(&self) -> u32 {
        if self.get_block_size().is_some() {
            4
        } else {
            1
        }
    }

    /// This gets the number of bits required to store a single pixel.  It is
    /// only defined for uncompressed formats
    fn get_bits_per_pixel(&self) -> Option<u8>;

    /// This gets a block compression format's block size, and is only defined
    /// for compressed formats
    fn get_block_size(&self) -> Option<u32>;

    /// Get the fourcc code for this format, if known
    fn get_fourcc(&self) -> Option<FourCC>;

    /// Returns true if the DX10 extention is required to use this format.
    fn requires_extension(&self) -> bool;

    /// This gets the minimum mipmap size in bytes. Even if they go all the way
    /// down to 1x1, there is a minimum number of bytes based on bits per pixel
    /// or blocksize.
    fn get_minimum_mipmap_size_in_bytes(&self) -> Option<u32> {
        if let Some(bpp) = self.get_bits_per_pixel() {
            Some((bpp as u32 + 7) / 8)
        } else {
            self.get_block_size()
        }
    }
}
