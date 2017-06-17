use ::zero::Zero;

#[derive(Copy, Clone)]
pub enum Color {
    RGBA{
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    },
    Scalar(u8),
}

#[derive(Copy, Clone)]
pub struct WaveformConfig {
    pub amp_min: f64,
    pub amp_max: f64,
    pub foreground: Color,
    pub background: Color,
}

#[derive(Copy, Clone)]
pub enum TimeRange {
    Seconds(f64, f64),
    Samples(usize, usize),
}

pub trait Sample: PartialOrd + Into<f64> + Copy + Zero {}
impl<T> Sample for T where T: PartialOrd + Into<f64> + Copy + Zero {}

pub struct SampleSequence<T: Sample> {
    pub data: Vec<T>,
    pub sample_rate: f64,
}

#[derive(Copy, Clone)]
pub struct MinMaxPair<T: Sample> {
    pub min: T,
    pub max: T,
}

pub struct MinMaxPairSequence<T: Sample> {
    pub data: Vec<MinMaxPair<T>>,
}

