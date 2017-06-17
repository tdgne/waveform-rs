# waveform

waveform is a set of (currently two) waveform image generators written in rust.

## Features

* Generation of raster images (outputs are `Vec<u8>`)
  * RGBA format images
  * Gray scale (scalar pixel) images
* Direct generation from a Vec of samples
* Faster indirect generation from cache

## Screenshot

```sh
cargo run --release --example waveform
```

![examples/waveform.rs](https://user-images.githubusercontent.com/29127111/27250722-dd579ff6-5370-11e7-99c2-7dc3e7705c14.png)

