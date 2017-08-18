//! Waveform image renderes meant to be used for audio visualization.

#[cfg(feature="rlibc")]
extern crate rlibc;

pub mod error;

pub mod zero;

pub mod misc;
pub use misc::{SampleSequence, WaveformConfig, Color, TimeRange, Sample};

#[macro_use]
mod macros;

pub mod binned;
pub use binned::BinnedWaveformRenderer;

pub mod direct;
pub use direct::DirectWaveformRenderer;

pub mod multi;
pub use multi::MultiWaveformRenderer;

