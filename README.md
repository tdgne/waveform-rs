# waveform

waveform is a set of (currently two) waveform image renderers written in Rust.
It is speed-oriented for heavy use.

This lib currently has no backends, so it directly renders to a `Vec<u8>`.

## Features

* Generation of bicolored raster images (outputs are `Vec<u8>`)
  * `RGBA` format images
  * Gray scale (`Scalar`) images for use as masks etc.
* Fast rendering from binned min/max amplitudes
* Multilevel binning for rendering in various resolutions.
* Time-range specification in either seconds (`f64`) or samples (`usize`)


## Screenshot

```sh
cargo run --release --example waveform
```

![examples/waveform.rs](https://user-images.githubusercontent.com/29127111/27250722-dd579ff6-5370-11e7-99c2-7dc3e7705c14.png)

