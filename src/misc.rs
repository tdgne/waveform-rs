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

/// Utility macro for accessing pixels
macro_rules! pixel {
    ($name:ident [ H ; $w:expr, $h:expr, $l:expr ; $x:expr , $y:expr , $i:expr ])
        => ($name[($x + $y * $w) * $l + $i]);
    ($name:ident [ V ; $w:expr, $h:expr, $l:expr ; $x:expr , $y:expr , $i:expr ])
        => ($name[($y + $x * $h) * $l + $i]);
    ($name:ident [ $w:expr, $h:expr, $l:expr ; $x:expr , $y:expr , $i:expr ])
        => (pixel!($name[H; $w, $h, $l; $x, $y, $i])); //
    ($name:ident [ H ; $w:expr, $h:expr ; $x:expr , $y:expr ])
        => (pixel!($name[H; $w, $h, 1; $x, $y, 0]));
    ($name:ident [ V ; $w:expr, $h:expr ; $x:expr , $y:expr ])
        => (pixel!($name[V; $w, $h, 1; $x, $y, 0]));
    ($name:ident [ $w:expr, $h:expr ; $x:expr , $y:expr ])
        => (pixel!($name[$w, $h, 1; $x, $y, 0]));
}


