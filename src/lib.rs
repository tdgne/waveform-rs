use std::error::Error;

mod error;
use error::InvalidSizeError;

mod zero;
use zero::Zero;

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
pub struct SimpleWaveformGenerator {
    pub sample_rate: f64,
    pub config: WaveformConfig,
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

struct MinMaxPair<T: Sample> {
    min: T,
    max: T,
}

struct MinMaxPairSequence<T: Sample> {
    data: Vec<MinMaxPair<T>>,
}

pub struct LightweightWaveformGenerator<T: Sample> {
    pub config: WaveformConfig,
    sample_rate: f64,
    bin_size: usize,
    minmax: MinMaxPairSequence<T>,
}

impl<T: Sample> LightweightWaveformGenerator<T> {
    pub fn new(samples: &SampleSequence<T>, bin_size: usize, config: WaveformConfig) -> Result<LightweightWaveformGenerator<T>, Box<Error>> {
        let mut data: Vec<MinMaxPair<T>> = Vec::new();
        let nb_samples = samples.data.len();

        if bin_size > nb_samples {
            return Err(Box::new(InvalidSizeError{var_name: "bin_size".to_string()}));
        }

        let nb_bins = nb_samples / bin_size;

        for x in 0..nb_bins {
            let mut min = samples.data[x*bin_size + 0];
            let mut max = samples.data[x*bin_size + 0];
            if bin_size > 1 {
                for i in 1..bin_size {
                    let idx = x * bin_size + i;
                    if idx >= nb_samples {
                        break;
                    }
                    let s = samples.data[idx];
                    if s > max {
                        max = s;
                    }else if s < min {
                        min = s;
                    }
                }
            }
            data.push(MinMaxPair{min: min, max: max});
        }
        let minmax = MinMaxPairSequence{data: data};
        Ok(Self{config: config, bin_size: bin_size, minmax: minmax, sample_rate: samples.sample_rate})
    }

    pub fn generate_vec(&mut self, range: TimeRange, shape: (usize, usize)) -> Option<Vec<u8>> {
        let (w, h) = shape;
        if w == 0 || h == 0 {
            return None;
        }

        let mut img = vec![0u8; w * h * 4];
        let (begin, end) = match range {
            TimeRange::Seconds(b, e) => ((b * self.sample_rate) as usize,
                                         (e * self.sample_rate) as usize),
            TimeRange::Samples(b, e) => (b, e),
        };
        let nb_samples = end - begin;
        let samples_per_pixel = (nb_samples as f64) / (w as f64);
        let bins_per_pixel = samples_per_pixel / (self.bin_size as f64);
        let bins_per_pixel_floor = bins_per_pixel.floor() as usize;
        let bins_per_pixel_ceil = bins_per_pixel.ceil() as usize;

        let offset_bin_idx = begin / self.bin_size;
        let mut start_bin_idx = offset_bin_idx;
        for x in 0..w {
            let inc = if x == 0 {
                bins_per_pixel_floor
            }else{
                if ((start_bin_idx - offset_bin_idx) as f64 + 1f64) / (x as f64) < bins_per_pixel {
                    bins_per_pixel_ceil
                }else{
                    bins_per_pixel_floor
                }
            };

            let mut min: T;
            let mut max: T;
            if start_bin_idx < self.minmax.data.len() - 1 {
                let ref d = self.minmax.data[start_bin_idx];
                min = d.min;
                max = d.max;
                let range_start = start_bin_idx;
                let range_end = if start_bin_idx + inc <= self.minmax.data.len() {
                    start_bin_idx + inc
                }else{
                    self.minmax.data.len()
                };
                for b in self.minmax.data[range_start..range_end].iter() {
                    if b.min < min { min = b.min }
                    if b.max > max { max = b.max }
                }
                start_bin_idx = range_end;
            }else{
                min = T::zero();
                max = T::zero();
            }

            for y in 0..h {
                let y_translated = ((h - y) as f64) / (h as f64) * (self.config.amp_max - self.config.amp_min) + self.config.amp_min;
                if y_translated < min.into() || y_translated > max.into() {
                    match self.config.background {
                        Color::RGBA{r, g, b, a} => {
                            img[4*(y*w+x) + 0] = r;
                            img[4*(y*w+x) + 1] = g;
                            img[4*(y*w+x) + 2] = b;
                            img[4*(y*w+x) + 3] = a;
                        },
                        Color::Scalar(a) => {
                            img[1*(y*w+x) + 0] = a;
                        }
                    }
                }else{
                    match self.config.foreground {
                        Color::RGBA{r, g, b, a} => {
                            img[4*(y*w+x) + 0] = r;
                            img[4*(y*w+x) + 1] = g;
                            img[4*(y*w+x) + 2] = b;
                            img[4*(y*w+x) + 3] = a;
                        },
                        Color::Scalar(a) => {
                            img[1*(y*w+x) + 0] = a;
                        }
                    }
                }
            }
        }

        Some(img)
    }
}



impl SimpleWaveformGenerator {
    pub fn generate_vec(&self, samples: &[f64], shape: (usize, usize)) -> Option<Vec<u8>> {
        let (w, h) = shape;
        if w == 0 || h == 0 {
            return None;
        }
        let mut img = vec![0u8; w * h * 4];
        let nb_samples = samples.len();
        let samples_per_pixel = nb_samples / w;

        let mut minmax = MinMaxPairSequence{data: Vec::with_capacity(w)};
        for x in 0..w {
            let mut min = samples[x*samples_per_pixel + 0];
            let mut max = samples[x*samples_per_pixel + 0];
            if samples_per_pixel > 1 {
                for i in 1..samples_per_pixel {
                    let idx = x * samples_per_pixel + i;
                    if idx >= nb_samples {
                        break;
                    }
                    let s = samples[idx];
                    if s > max {
                        max = s;
                    }
                    if s < min {
                        min = s;
                    }
                }
            }
            minmax.data.push(MinMaxPair{min: min, max: max});
        }

        match self.config.background {
            Color::RGBA{r, g, b, a} => {
                for y in 0..h {
                    let y_translated = ((h - y) as f64) / (h as f64) * (self.config.amp_max - self.config.amp_min) + self.config.amp_min;
                    for x in 0..w {
                        if y_translated < minmax.data[x].min || y_translated > minmax.data[x].max {
                            img[4*(y*w+x) + 0] = r;
                            img[4*(y*w+x) + 1] = g;
                            img[4*(y*w+x) + 2] = b;
                            img[4*(y*w+x) + 3] = a;
                        }else{
                            img[4*(y*w+x) + 0] = r;
                            img[4*(y*w+x) + 1] = g;
                            img[4*(y*w+x) + 2] = b;
                            img[4*(y*w+x) + 3] = a;
                        }
                    }
                }
            },
            Color::Scalar(a) => {
                for y in 0..h {
                    let y_translated = ((h - y) as f64) / (h as f64) * (self.config.amp_max - self.config.amp_min) + self.config.amp_min;
                    for x in 0..w {
                        if y_translated < minmax.data[x].min || y_translated > minmax.data[x].max {
                            img[1*(y*w+x) + 0] = a;
                        }else{
                            img[1*(y*w+x) + 0] = a;
                        }
                    }
                }
            },
        }
        Some(img)
    }
}
