//! Waveform image generators meant to be used for audio visualization.

pub mod error;

pub mod zero;

pub mod misc;
pub use misc::{SampleSequence, WaveformConfig, Color, TimeRange, Sample};

pub mod binned;
pub use binned::BinnedWaveformGenerator;

pub mod direct;
pub use direct::DirectWaveformGenerator;

