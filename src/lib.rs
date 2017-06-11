use std::collections::HashMap;

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone)]
pub struct WaveformConfig {
    pub amp_min: f64,
    pub amp_max: f64,
    pub foreground: Color,
    pub background: Color,
}

#[derive(Clone)]
pub struct SimpleWaveformGenerator {
    pub sample_rate: f64,
    pub config: WaveformConfig,
}


pub enum TimeRange {
    Seconds(f64, f64),
    Samples(usize, usize),
}

pub trait Sample: PartialOrd + Into<f64> + Copy {}
impl<T> Sample for T where T: PartialOrd + Into<f64> + Copy {}

pub struct SampleSequence<T: Sample> {
    pub data: Vec<T>,
    pub sample_rate: f64,
    pub range: TimeRange,
}

struct MinMaxPair<T: Sample> {
    min: T,
    max: T,
}

struct MinMaxPairSequence<T: Sample> {
    data: Vec<MinMaxPair<T>>,
    range: TimeRange,
}

pub struct CachedWaveformGenerator<T: Sample> {
    pub samples: SampleSequence<T>,
    pub config: WaveformConfig,
    minmax: HashMap<usize, MinMaxPairSequence<T>>, // key: samples_per_pixel, val: pairs of min/max values
}

impl<T: Sample> CachedWaveformGenerator<T> {
    pub fn new(samples: SampleSequence<T>, config: WaveformConfig) -> CachedWaveformGenerator<T> {
        Self{samples: samples, minmax: HashMap::new(), config: config}
    }
    pub fn generate_vec(&mut self, range: TimeRange, shape: (usize, usize)) -> Option<Vec<u8>> {
        let (w, h) = shape;
        if w == 0 || h == 0 {
            return None;
        }
        let mut img = vec![0u8; w * h * 4];
        let nb_samples = self.samples.data.len();
        let samples_per_pixel = nb_samples / w;

        let minmax_cache_exists = match self.minmax.get(&samples_per_pixel) {
            Some(_) => true,
            None => false,
        };

        if !minmax_cache_exists {
            self.minmax.insert(samples_per_pixel,
                               MinMaxPairSequence{data: Vec::with_capacity(w), range: range});
            let ref mut minmaxvec = self.minmax.get_mut(&samples_per_pixel).unwrap();
            for x in 0..w {
                let mut min = self.samples.data[x*samples_per_pixel + 0];
                let mut max = self.samples.data[x*samples_per_pixel + 0];
                if samples_per_pixel > 1 {
                    for i in 1..samples_per_pixel {
                        let idx = x * samples_per_pixel + i;
                        if idx >= nb_samples {
                            break;
                        }
                        let s = self.samples.data[idx];
                        if s > max {
                            max = s;
                        }else if s < min {
                            min = s;
                        }
                    }
                }
                minmaxvec.data.push(MinMaxPair{min: min, max: max});
            }
        }

        let ref cache = self.minmax.get(&samples_per_pixel).unwrap().data;
        for x in 0..w {
            let MinMaxPair{min: min, max: max} = cache[x];
            for y in 0..h {
                let y_translated = ((h - y) as f64) / (h as f64) * (self.config.amp_max - self.config.amp_min) + self.config.amp_min;
                if y_translated < min.into() || y_translated > max.into() {
                    img[4*(y*w+x) + 0] = self.config.background.r;
                    img[4*(y*w+x) + 1] = self.config.background.g;
                    img[4*(y*w+x) + 2] = self.config.background.b;
                    img[4*(y*w+x) + 3] = self.config.background.a;
                }else{
                    img[4*(y*w+x) + 0] = self.config.foreground.r;
                    img[4*(y*w+x) + 1] = self.config.foreground.g;
                    img[4*(y*w+x) + 2] = self.config.foreground.b;
                    img[4*(y*w+x) + 3] = self.config.foreground.a;
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
            for y in 0..h {
                let y_translated = ((h - y) as f64) / (h as f64) * (self.config.amp_max - self.config.amp_min) + self.config.amp_min;
                if y_translated < min || y_translated > max {
                    img[4*(y*w+x) + 0] = self.config.background.r;
                    img[4*(y*w+x) + 1] = self.config.background.g;
                    img[4*(y*w+x) + 2] = self.config.background.b;
                    img[4*(y*w+x) + 3] = self.config.background.a;
                }else{
                    img[4*(y*w+x) + 0] = self.config.foreground.r;
                    img[4*(y*w+x) + 1] = self.config.foreground.g;
                    img[4*(y*w+x) + 2] = self.config.foreground.b;
                    img[4*(y*w+x) + 3] = self.config.foreground.a;
                }
            }
        }
        Some(img)
    }
}
