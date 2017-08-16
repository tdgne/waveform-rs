use std::collections::HashMap;
use std::error::Error;

use super::misc::*;
use super::binned::BinnedWaveformRenderer;

pub struct MultiWaveformRenderer<T: Sample> {
    pub binned: HashMap<usize, BinnedWaveformRenderer<T>>,
}

impl<T: Sample> MultiWaveformRenderer<T> {
    pub fn new(samples: &SampleSequence<T>, bin_sizes: &Vec<usize>, config: WaveformConfig) -> Result<Self, Box<Error>> {
        let mut r = MultiWaveformRenderer{binned: HashMap::new()};
        let mut bss = bin_sizes.clone();
        bss.sort();
        for bs in bss.iter() {
            r.binned.insert(*bs, try!(BinnedWaveformRenderer::new(samples, *bs, config)));
        }
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::MultiWaveformRenderer;
    use ::misc::*;

    #[test]
    fn multi_new() {
        let data = vec![0f64; 50000];
        let sample_rate = 44100f64;
        let ss = SampleSequence{data: &data[..], sample_rate};
        let foreground = Color::RGBA{r: 255, g: 0, b: 0, a: 255};
        let background = Color::RGBA{r: 0, g: 0, b: 0, a: 0};
        let config = WaveformConfig{amp_min: -100f64, amp_max: 100f64, foreground, background};
        let bss = vec![10, 50, 100];
        let mwr = MultiWaveformRenderer::new(&ss, &bss, config).unwrap();
        for bs in bss.iter() {
            assert_eq!(mwr.binned.get(bs).unwrap().get_bin_size(), *bs);
            assert_eq!(mwr.binned.get(bs).unwrap().get_sample_rate(), sample_rate);
        }
    }
}
