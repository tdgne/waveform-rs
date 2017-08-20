//! Waveform image renderes meant to be used for audio visualization.

#[cfg(feature = "rlibc")]
extern crate rlibc;

#[cfg(feature = "ocl")]
extern crate ocl;

pub mod error;

pub mod zero;

pub mod misc;
pub use misc::{Color, Sample, SampleSequence, TimeRange, WaveformConfig};

#[macro_use]
mod macros;

pub mod binned;
pub use binned::BinnedWaveformRenderer;

#[deprecated]
pub mod direct;

pub mod multi;
pub use multi::MultiWaveformRenderer;

