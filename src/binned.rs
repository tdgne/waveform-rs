use std::error::Error;
use std::cmp;
use error::InvalidSizeError;
use misc::*;
use rlibc;


/// A fast "binned" waveform renderer.
///
/// Minimum / maximum amplitude values are binned to reduce
/// calculation and memory usage.
pub struct BinnedWaveformRenderer<T: Sample> {
    pub config: WaveformConfig,
    sample_rate: f64,
    bin_size: usize,
    minmax: MinMaxPairSequence<T>,
}

impl<T: Sample> BinnedWaveformRenderer<T> {
    /// The constructor.
    ///
    /// # Arguments
    ///
    /// * `samples` - The samples that will be used to calculate binned min / max values.
    ///               It must also contain the sample rate that is used by
    ///               `BinnedWaveformRenderer` to render images when given a
    ///               `TimeRange::Seconds`.
    /// * `bin_size` - The size of the bins which the min / max values will be binned
    ///                into.
    /// * `config` - See `WaveformConfig`.
    pub fn new(
        samples: &SampleSequence<T>,
        bin_size: usize,
        config: WaveformConfig,
    ) -> Result<BinnedWaveformRenderer<T>, Box<Error>> {
        let mut data: Vec<MinMaxPair<T>> = Vec::new();
        let nb_samples = samples.data.len();

        if bin_size > nb_samples {
            return Err(Box::new(
                InvalidSizeError { var_name: "bin_size".to_string() },
            ));
        }

        let nb_bins = nb_samples / bin_size;

        for x in 0..nb_bins {
            let mut min = samples.data[x * bin_size + 0];
            let mut max = samples.data[x * bin_size + 0];
            if bin_size > 1 {
                for i in 1..bin_size {
                    let idx = x * bin_size + i;
                    if idx >= nb_samples {
                        break;
                    }
                    let s = samples.data[idx];
                    if s > max {
                        max = s;
                    } else if s < min {
                        min = s;
                    }
                }
            }
            data.push(MinMaxPair { min: min, max: max });
        }
        let minmax = MinMaxPairSequence { data: data };
        Ok(Self {
            config: config,
            bin_size: bin_size,
            minmax: minmax,
            sample_rate: samples.sample_rate,
        })
    }


    /// Generates an image as a `Vec<u8>`.
    ///
    /// # Arguments
    ///
    /// * `range` - The samples within this `TimeRange` will be rendered.
    /// * `shape` - The `(width, height)` of the resulting image.
    pub fn render_vec(&mut self, range: TimeRange, shape: (usize, usize)) -> Option<Vec<u8>> {
        let mut bg_is_scalar = false;
        let mut fg_is_scalar = false;
        if let Color::Scalar(_) = self.config.background {
            bg_is_scalar = true;
        }
        if let Color::Scalar(_) = self.config.foreground {
            fg_is_scalar = true;
        }
        if bg_is_scalar ^ fg_is_scalar {
            panic!("Color formats of background and foreground are inconsistent!");
        }

        let (w, h) = shape;
        if w == 0 || h == 0 {
            return None;
        }

        let mut img = vec![0u8; w * h * 4];
        let (begin, end) = match range {
            TimeRange::Seconds(b, e) => (
                (b * self.sample_rate) as usize,
                (e * self.sample_rate) as usize,
            ),
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
            } else {
                if ((start_bin_idx - offset_bin_idx) as f64 + 1f64) / (x as f64) < bins_per_pixel {
                    bins_per_pixel_ceil
                } else {
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
                } else {
                    self.minmax.data.len()
                };
                for b in self.minmax.data[range_start..range_end].iter() {
                    if b.min < min {
                        min = b.min
                    }
                    if b.max > max {
                        max = b.max
                    }
                }
                start_bin_idx = range_end;
            } else {
                min = T::zero();
                max = T::zero();
            }

            let scale = 1f64 / (self.config.amp_max - self.config.amp_min) * (h as f64);
            let min_translated: usize = h - cmp::max(0, cmp::min(h, ((min.into() - self.config.amp_min) * scale).floor() as usize));
            let max_translated: usize = h - cmp::max(0, cmp::min(h, ((max.into() - self.config.amp_min) * scale).floor() as usize));

            match (self.config.background, self.config.foreground) {
                (Color::RGBA{r:br, g:bg, b:bb, a:ba}, Color::RGBA{r:fr, g:fg, b:fb, a:fa})
                    => {
                        let bg_colors: [u8; 4] = [br, bg, bb, ba];
                        let fg_colors: [u8; 4] = [fr, fg, fb, fa];
                        unsafe {
                            for y in 0..max_translated {
                                    rlibc::memcpy(
                                        &mut pixel!(img[w, h, 4; x, y, 0]) as _,
                                        &bg_colors[0] as _,
                                        4
                                        );
                            }
                            for y in max_translated..min_translated {
                                    rlibc::memcpy(
                                        &mut pixel!(img[w, h, 4; x, y, 0]) as _,
                                        &fg_colors[0] as _,
                                        4
                                        );
                            }
                            for y in min_translated..h {
                                    rlibc::memcpy(
                                        &mut pixel!(img[w, h, 4; x, y, 0]) as _,
                                        &bg_colors[0] as _,
                                        4
                                        );
                            }
                        }
                    },
                (Color::Scalar(ba), Color::Scalar(fa))
                    => {
                        for y in 0..max_translated {
                            pixel!(img[w, h; x, y]) = ba;
                        }
                        for y in max_translated..min_translated {
                            pixel!(img[w, h; x, y]) = fa;
                        }
                        for y in min_translated..h {
                            pixel!(img[w, h; x, y]) = ba;
                        }
                    },
                (_, _) => unreachable!(),
            }
        }

        Some(img)
    }

    pub fn get_bin_size(&self) -> usize {
        self.bin_size
    }
    pub fn get_sample_rate(&self) -> f64 {
        self.sample_rate
    }
}
