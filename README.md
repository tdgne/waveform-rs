# waveform

[![crates.io](https://img.shields.io/crates/v/waveform.svg)](https://crates.io/crates/waveform) [![docs.rs](https://docs.rs/waveform/badge.svg)](https://docs.rs/waveform/) [![Build Status](https://travis-ci.org/tdgne/waveform-rs.svg?branch=master)](https://travis-ci.org/tdgne/waveform-rs)

waveform is a set of waveform image renderers written in Rust.

It is speed-oriented for heavy use.

## Features

* Generation of bicolored raster images (outputs are either returned as `Vec<u8>`s or written into a slice)
  * RGB (`Vector3`) or RGBA (`Vector4`) format images
  * Gray scale (`Scalar`) images for use as masks etc.
* Fast rendering from binned min/max amplitudes
* Multilevel binning for rendering in various resolutions
* Time-range specification in either seconds (`f64`) or samples (`usize`)

## Some TODOs

* Cached rendering
* Guarantee thread safety (it probably is...)
* Memory/time optimizations

Requests and contributions are welcome!

## Screenshot

```sh
# Demonstrates rendering using a single BinnedWaveformRenderer.
cargo run --features "example-gui" --example binned
```

```sh
# The same but by using a MultiWaveformRenderer, which is
# a combination of multiple BinnedWaveformRenderers.
cargo run --features "example-gui" --example multi
```

![examples/waveform.rs](https://user-images.githubusercontent.com/29127111/27250722-dd579ff6-5370-11e7-99c2-7dc3e7705c14.png)


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

