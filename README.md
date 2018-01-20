# ddsfile

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)

[ddsfile on crates.io](https://crates.io/crates/ddsfile)

[Documentation](https://docs.rs/ddsfile)

This library is for parsing and composing Microsoft DirectDraw Surface (.DDS)
files. Such files hold texture data, originally for DirectX, but other drawing
APIs such as OpenGL and Vulkan can use the texture data. Many asset conditioning
pipelines deal in this format only, so even if you are working with OpenGL or
Vulkan you probably still need to handle .DDS files.

This library supports mipmapped textures, volume textures, texture arrays,
cube maps, compressed texture formats (DXTn / BCn) and the DirectX 10 extension
header. Both the older D3DFormat and the newer DxgiFormat are supported, as
well as files with the format undefined (whenever enough data is available to
do so).

This library deals primarily with the *container envelope*.  The texture data
itself is mostly opaque.  However, some data is available from the headers
about the texture data, including:

* The format
* The width, height, and depth
* The bits per pixel, pitch and stride
* The number of mipmap levels, if any
* The minimum size in bytes of a mipmap level
* The number of array layers, if any
* RGBA bitmasks for uncompressed formats (only available for older D3DFormats
  currently)
* The block size for compressed formats
* Several flags including CUBEMAP and LUMINANCE

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed under the MIT license without
any additional terms or conditions.
