//! Waveform image renderes meant to be used for audio visualization.

pub mod error;

pub mod zero;

pub mod misc;
pub use misc::{SampleSequence, WaveformConfig, Color, TimeRange, Sample};

pub mod binned;
pub use binned::BinnedWaveformRenderer;

pub mod multi;
pub use multi::MultiWaveformRenderer;

pub mod direct;
pub use direct::DirectWaveformRenderer;
