# waveform

[crates.io](https://crates.io/crates/waveform), [Documentation](https://docs.rs/waveform/0.1.0/waveform/)

waveform is a set of waveform image renderers written in Rust.

It is speed-oriented for heavy use.

## Features

* Generation of bicolored raster images (outputs are `Vec<u8>`)
  * `RGBA` format images
  * Gray scale (`Scalar`) images for use as masks etc.
* Fast rendering from binned min/max amplitudes
* Multilevel binning for rendering in various resolutions
* Time-range specification in either seconds (`f64`) or samples (`usize`)

## Some TODOs

* Direct writing into given slices
* Cached rendering
* Guarantee thread safety (it probably is...)
* Memory/time optimizations

Requests and contributions are welcome!

## Screenshot

```sh
cargo run --release --example waveform
```

![examples/waveform.rs](https://user-images.githubusercontent.com/29127111/27250722-dd579ff6-5370-11e7-99c2-7dc3e7705c14.png)



