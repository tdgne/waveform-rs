//! Waveform image renderes meant to be used for audio visualization.

extern crate rlibc;

pub mod error;

pub mod zero;

#[macro_use]
pub mod misc;
pub use misc::{SampleSequence, WaveformConfig, Color, TimeRange, Sample};

pub mod binned;
pub use binned::BinnedWaveformRenderer;

pub mod direct;
pub use direct::DirectWaveformRenderer;

pub mod multi;
pub use multi::MultiWaveformRenderer;

