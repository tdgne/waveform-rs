use zero::Zero;
use error::InconsistentFormatError;
use std::error::Error;

/// Color specifiers.
#[derive(Copy, Clone)]
pub enum Color {
    /// RGBA format.
    RGBA { r: u8, g: u8, b: u8, a: u8 },
    /// A format with only one value per pixel, or gray scale in other words.
    Scalar(u8),
}

/// Configurations for image generators.
///
/// It contains the following information:
///
///  * Range of the amplitudes to be rendered
///  * Foreground and background `Color`s to be used
#[derive(Copy, Clone)]
pub struct WaveformConfig {
    pub amp_min: f64,
    pub amp_max: f64,
    foreground: Color,
    background: Color,
}

impl WaveformConfig {
    fn check_color_consistency(c1: Color, c2: Color) -> Result<(), Box<InconsistentFormatError>> {
        let mut c1_is_scalar = false;
        let mut c2_is_scalar = false;
        if let Color::Scalar(_) = c1 {
            c1_is_scalar = true;
        }
        if let Color::Scalar(_) = c2 {
            c2_is_scalar = true;
        }

        if c1_is_scalar ^ c2_is_scalar {
            return Err(Box::new(InconsistentFormatError));
        }

        Ok(())
    }

    /// The constructor.
    ///
    /// # Arguments
    /// * `amp_min` - Minimum value of amplitude to be rendered
    /// * `amp_max` - Maximum value of amplitude to be rendered
    /// * `foreground` - Foreground `Color` of the image, format must be consistent with background.
    /// * `background` - Background `Color` of the image, format must be consistent with foreground.
    pub fn new(amp_min: f64, amp_max: f64, foreground: Color, background: Color) -> Result<Self, Box<Error>> {
        match Self::check_color_consistency(background, foreground) {
            Err(e) => return Err(e),
            _ => (),
        }

        Ok(Self {
            amp_min,
            amp_max,
            background,
            foreground,
        })
    }

    pub fn get_background(&self) -> Color {
        self.background
    }
    pub fn get_foreground(&self) -> Color {
        self.foreground
    }

    /// Sets `Color`s.
    ///
    /// # Arguments
    /// * `foreground` - Foreground `Color` of the image, format must be consistent with background.
    /// * `background` - Background `Color` of the image, format must be consistent with foreground.
    pub fn set_colors(&mut self, background: Color, foreground: Color) -> Result<(), Box<Error>> {
        match Self::check_color_consistency(background, foreground) {
            Err(e) => return Err(e),
            _ => (),
        }

        self.background = background;
        self.foreground = foreground;

        Ok(())
    }
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

/// A reference to a `slice` of `Sample`s
/// (which describe a wave) combined with its sample rate.
pub struct SampleSequence<'a, T: Sample + 'a> {
    pub data: &'a [T],
    pub sample_rate: f64,
}

/// A pair of a minimum and maximum amplitude values for internal use.
#[derive(Copy, Clone)]
pub struct MinMaxPair<T: Sample> {
    pub min: T,
    pub max: T,
}

pub struct MinMaxPairSequence<T: Sample> {
    pub data: Vec<MinMaxPair<T>>,
}
