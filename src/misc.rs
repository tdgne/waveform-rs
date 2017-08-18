use zero::Zero;

/// Color specifiers.
#[derive(Copy, Clone)]
pub enum Color {
    /// RGBA format.
    RGBA { r: u8, g: u8, b: u8, a: u8 },
    /// A format with only one value per pixel, or gray scale in other words.
    Scalar(u8),
}

/// Configurations for image generators.
#[derive(Copy, Clone)]
pub struct WaveformConfig {
    /// Minimum amplitude to be plotted.
    pub amp_min: f64,

    /// Maximum amplitude to be plotted.
    pub amp_max: f64,

    /// Foreground color of the image, format must be consistent with background.
    pub foreground: Color,

    /// Background color of the image, format must be consistent with foreground.
    pub background: Color,
}

impl Default for WaveformConfig {
    fn default() -> Self {
        Self {
            amp_min: -1f64,
            amp_max: 1f64,
            foreground: Color::Scalar(255),
            background: Color::Scalar(0),
        }
    }
}

/// Time range specifiers used to determine which part of the wave to plot.
#[derive(Copy, Clone)]
pub enum TimeRange {
    Seconds(f64, f64),
    Samples(usize, usize),
}

/// A sample.
pub trait Sample: PartialOrd + Into<f64> + Copy + Zero {}
impl<T> Sample for T
where
    T: PartialOrd + Into<f64> + Copy + Zero,
{
}

/// A sequence of `Sample`s (a wave) combined with sample rate information.
pub struct SampleSequence<'a, T: Sample + 'a> {
    pub data: &'a [T],
    pub sample_rate: f64,
}

/// A pair of a minimum and maximum amplitude value for internal use.
#[derive(Copy, Clone)]
pub struct MinMaxPair<T: Sample> {
    pub min: T,
    pub max: T,
}

pub struct MinMaxPairSequence<T: Sample> {
    pub data: Vec<MinMaxPair<T>>,
}

