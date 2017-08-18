use misc::*;

#[derive(Copy, Clone)]
pub struct DirectWaveformRenderer {
    pub sample_rate: f64,
    pub config: WaveformConfig,
}

impl DirectWaveformRenderer {
    /// Generates an image as a `Vec<u8>` directly from the given samples.
    ///
    /// # Arguments
    ///
    /// * `samples` - The `Sample`s that will be used to render the image.
    /// * `shape` - The `(width, height)` of the resulting image.
    pub fn render_vec<T: Sample>(&self, samples: &[T], shape: (usize, usize)) -> Option<Vec<u8>> {
        let (w, h) = shape;
        if w == 0 || h == 0 {
            return None;
        }
        let mut img = vec![0u8; w * h * 4];
        let nb_samples = samples.len();
        let samples_per_pixel = nb_samples / w;

        // Unlike BinnedWaveformRenderer, the minmax values corresponding to
        // each horizontal pixel is calculated beforehand, for the same reasons
        // discussed later in these comments.
        let mut minmax = MinMaxPairSequence {
            data: Vec::with_capacity(w),
        };
        for x in 0..w {
            let mut min = samples[x * samples_per_pixel + 0];
            let mut max = samples[x * samples_per_pixel + 0];
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
            minmax.data.push(MinMaxPair { min: min, max: max });
        }

        // Unlike BinnedWaveformRenderer, the `match` is outside the `for`s
        // because it's faster this way.
        // I've also tried it in BinnedWaveformRenderer but it didn't make a
        // significant improvement in speed, so it's left that way.
        match (self.config.get_background(), self.config.get_foreground()) {
            (
                Color::RGBA {
                    r: br,
                    g: bg,
                    b: bb,
                    a: ba,
                },
                Color::RGBA {
                    r: fr,
                    g: fg,
                    b: fb,
                    a: fa,
                },
            ) => for y in 0..h {
                let y_translated = ((h - y) as f64) / (h as f64) * (self.config.amp_max - self.config.amp_min) + self.config.amp_min;
                for x in 0..w {
                    if y_translated < minmax.data[x].min.into() || y_translated > minmax.data[x].max.into() {
                        img[4 * (y * w + x) + 0] = br;
                        img[4 * (y * w + x) + 1] = bg;
                        img[4 * (y * w + x) + 2] = bb;
                        img[4 * (y * w + x) + 3] = ba;
                    } else {
                        img[4 * (y * w + x) + 0] = fr;
                        img[4 * (y * w + x) + 1] = fg;
                        img[4 * (y * w + x) + 2] = fb;
                        img[4 * (y * w + x) + 3] = fa;
                    }
                }
            },
            (Color::Scalar(ba), Color::Scalar(fa)) => for y in 0..h {
                let y_translated = ((h - y) as f64) / (h as f64) * (self.config.amp_max - self.config.amp_min) + self.config.amp_min;
                for x in 0..w {
                    if y_translated < minmax.data[x].min.into() || y_translated > minmax.data[x].max.into() {
                        img[1 * (y * w + x) + 0] = ba;
                    } else {
                        img[1 * (y * w + x) + 0] = fa;
                    }
                }
            },
            _ => {
                panic!("Color formats of background and foreground are inconsistent!");
            }
        }
        Some(img)
    }
}
