use std::ops::Range;
use std::collections::HashMap;

pub struct SimpleWaveformGenerator {
    pub sample_rate: f64,
    pub amp_min: f64,
    pub amp_max: f64,
    pub foreground: Color,
    pub background: Color,
}

pub struct CachedWaveformGenerator {
    samples: Vec<f64>,
    sample_rate: f64,
    amp_min: f64,
    amp_max: f64,
    minmax: HashMap<usize, Vec<MinMaxPair>>, // key: samples_per_pixel, val: pairs of min/max values
    foreground: Color,
    background: Color,
}

struct MinMaxPair {
    min: f64,
    max: f64,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl CachedWaveformGenerator {
    pub fn new(samples: Vec<f64>, sample_rate: f64, amp_min: f64, amp_max: f64, foreground: Color, background: Color) -> Self {
        Self{samples: samples, sample_rate: sample_rate, amp_min: amp_min, amp_max: amp_max,
                        minmax: HashMap::new(), background: background, foreground: foreground}
    }
    pub fn generate_vec(&mut self, range: (f64, f64), shape: (usize, usize)) -> Option<Vec<u8>> {
        let (w, h) = shape;
        if w == 0 || h == 0 {
            return None;
        }
        let mut img = vec![0u8; w * h * 4];
        let nb_samples = self.samples.len();
        let samples_per_pixel = nb_samples / w;

        let minmax_cache_exists = match self.minmax.get(&samples_per_pixel) {
            Some(_) => true,
            None => false,
        };

        if !minmax_cache_exists {
            self.minmax.insert(samples_per_pixel, Vec::with_capacity(w));
            let minmaxvec = self.minmax.get_mut(&samples_per_pixel).unwrap();
            for x in 0..w {
                let mut min = self.samples[x*samples_per_pixel + 0];
                let mut max = self.samples[x*samples_per_pixel + 0];
                if samples_per_pixel > 1 {
                    for i in 1..samples_per_pixel {
                        let idx = x * samples_per_pixel + i;
                        if idx >= nb_samples {
                            break;
                        }
                        let s = self.samples[idx];
                        if s > max {
                            max = s;
                        }else if s < min {
                            min = s;
                        }
                    }
                }
                minmaxvec.push(MinMaxPair{min: min, max: max});
            }
        }

        let cache = self.minmax.get(&samples_per_pixel).unwrap();
        for x in 0..w {
            let MinMaxPair{min: min, max: max} = cache[x];
            for y in 0..h {
                let y_translated = ((h - y) as f64) / (h as f64) * (self.amp_max - self.amp_min) + self.amp_min;
                if y_translated < min || y_translated > max {
                    img[4*(y*w+x) + 0] = self.background.r;
                    img[4*(y*w+x) + 1] = self.background.g;
                    img[4*(y*w+x) + 2] = self.background.b;
                    img[4*(y*w+x) + 3] = self.background.a;
                }else{
                    img[4*(y*w+x) + 0] = self.foreground.r;
                    img[4*(y*w+x) + 1] = self.foreground.g;
                    img[4*(y*w+x) + 2] = self.foreground.b;
                    img[4*(y*w+x) + 3] = self.foreground.a;
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
                let y_translated = ((h - y) as f64) / (h as f64) * (self.amp_max - self.amp_min) + self.amp_min;
                if y_translated < min || y_translated > max {
                    img[4*(y*w+x) + 0] = self.background.r;
                    img[4*(y*w+x) + 1] = self.background.g;
                    img[4*(y*w+x) + 2] = self.background.b;
                    img[4*(y*w+x) + 3] = self.background.a;
                }else{
                    img[4*(y*w+x) + 0] = self.foreground.r;
                    img[4*(y*w+x) + 1] = self.foreground.g;
                    img[4*(y*w+x) + 2] = self.foreground.b;
                    img[4*(y*w+x) + 3] = self.foreground.a;
                }
            }
        }
        Some(img)
    }
}
