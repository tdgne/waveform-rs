use std::collections::HashMap;
use std::error::Error;

use super::misc::*;
use super::binned::BinnedWaveformRenderer;

pub struct MultiWaveformRenderer<T: Sample> {
    pub binned: HashMap<usize, BinnedWaveformRenderer<T>>,
    sample_rate: f64,
}

impl<T: Sample> MultiWaveformRenderer<T> {
    pub fn new(samples: &SampleSequence<T>, bin_sizes: &Vec<usize>, config: WaveformConfig) -> Result<Self, Box<Error>> {
        let mut r = MultiWaveformRenderer{binned: HashMap::new(), sample_rate: samples.sample_rate};
        let mut bss = bin_sizes.clone();
        bss.sort();
        for bs in bss.iter() {
            r.binned.insert(*bs, try!(BinnedWaveformRenderer::new(samples, *bs, config)));
        }
        Ok(r)
    }

    pub fn render_vec(&mut self, range: TimeRange, shape: (usize, usize)) -> Option<Vec<u8>> {
        let (w, h) = shape;
        if w == 0 || h == 0 {
            return None;
        }

        let (begin, end) = match range {
            TimeRange::Seconds(b, e) => (
                (b * self.sample_rate) as usize,
                (e * self.sample_rate) as usize,
            ),
            TimeRange::Samples(b, e) => (b, e),
        };

        let samples_per_pixel = ((end - begin) as f64) / (shape.0 as f64);

        let mut bin_sizes: Vec<usize> = self.binned.keys().map(|x| *x).collect();
        if bin_sizes.len() == 0 {
            return None;
        }

        bin_sizes.sort();
        let mut bin_size = bin_sizes[0];
        for bs in bin_sizes.iter() {
            if (*bs as f64) <= samples_per_pixel {
                bin_size = *bs;
            }else{
                break;
            }
        }

        self.binned.get_mut(&bin_size).unwrap().render_vec(range, shape)
    }
}

#[cfg(test)]
mod tests {
    use super::MultiWaveformRenderer;
    use ::misc::*;

    #[test]
    fn multi() {
        let data = vec![0f64; 50000];
        let sample_rate = 44100f64;
        let ss = SampleSequence{data: &data[..], sample_rate};
        let foreground = Color::RGBA{r: 255, g: 0, b: 0, a: 255};
        let background = Color::RGBA{r: 0, g: 0, b: 0, a: 0};
        let config = WaveformConfig{amp_min: -100f64, amp_max: 100f64, foreground, background};
        let bss = vec![10, 50, 100];
        let mut mwr = MultiWaveformRenderer::new(&ss, &bss, config).unwrap();

        for bs in bss.iter() {
            assert_eq!(mwr.binned.get(bs).unwrap().get_bin_size(), *bs);
            assert_eq!(mwr.binned.get(bs).unwrap().get_sample_rate(), sample_rate);
        }

        mwr.render_vec(TimeRange::Seconds(0f64, 1f64), (1000, 100)).unwrap();
    }
}