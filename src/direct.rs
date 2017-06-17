use ::misc::*;

#[derive(Copy, Clone)]
pub struct DirectWaveformGenerator {
    pub sample_rate: f64,
    pub config: WaveformConfig,
}

impl DirectWaveformGenerator {
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

        match (self.config.background, self.config.foreground) {
            (Color::RGBA{r: br, g: bg, b: bb, a: ba},
             Color::RGBA{r: fr, g: fg, b: fb, a: fa}) => {
                for y in 0..h {
                    let y_translated = ((h - y) as f64) / (h as f64) * (self.config.amp_max - self.config.amp_min) + self.config.amp_min;
                    for x in 0..w {
                        if y_translated < minmax.data[x].min || y_translated > minmax.data[x].max {
                            img[4*(y*w+x) + 0] = br;
                            img[4*(y*w+x) + 1] = bg;
                            img[4*(y*w+x) + 2] = bb;
                            img[4*(y*w+x) + 3] = ba;
                        }else{
                            img[4*(y*w+x) + 0] = fr;
                            img[4*(y*w+x) + 1] = fg;
                            img[4*(y*w+x) + 2] = fb;
                            img[4*(y*w+x) + 3] = fa;
                        }
                    }
                }
            },
            (Color::Scalar(ba), Color::Scalar(fa)) => {
                for y in 0..h {
                    let y_translated = ((h - y) as f64) / (h as f64) * (self.config.amp_max - self.config.amp_min) + self.config.amp_min;
                    for x in 0..w {
                        if y_translated < minmax.data[x].min || y_translated > minmax.data[x].max {
                            img[1*(y*w+x) + 0] = ba;
                        }else{
                            img[1*(y*w+x) + 0] = fa;
                        }
                    }
                }
            },
            _ => {
                panic!();
            }
        }
        Some(img)
    }
}
