# waveform

waveform is a set of (currently two) waveform image generators written in rust.

## Features

* Generation of raster images (outputs are `Vec<u8>`)
  * RGBA format images
  * Gray scale (scalar pixel) images
* Direct generation from a Vec of samples
* Faster indirect generation from cache
