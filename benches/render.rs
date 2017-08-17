#![feature(test)]
extern crate test;
extern crate waveform;

use test::Bencher;
use waveform::*;

fn gen_samples() -> Vec<f64> {
    let mut samples: Vec<f64> = Vec::new();
    for t in 0..441000 {
        // 10 seconds
        samples.push(
            ((t as f64) / 100f64 * 2f64 * 3.1415f64).sin() *
                ((t as f64) / 10000f64 * 2f64 * 3.1415f64).sin(),
        );
    }
    samples
}

fn gen_config() -> WaveformConfig {
    WaveformConfig {
        amp_max: 1f64,
        amp_min: -1f64,
        background: Color::RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
        foreground: Color::RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        },
    }
}

#[bench]
fn bench_binned(b: &mut Bencher) {
    let width = 1000usize; // The width of the rendered image.
    let height = 100usize; // The height of the rendered image.
    let bin_size = 10usize; // Bin size for BinnedWaveformRenderer

    let samples: Vec<f64> = gen_samples();

    let config = gen_config();

    let mut wfg = BinnedWaveformRenderer::new(
            &SampleSequence {
                data: &samples[..],
                sample_rate: 44100f64,
            },
            bin_size,
            config.clone(),
        ).unwrap();


    b.iter(|| {
        wfg.render_vec(TimeRange::Seconds(0f64, 10f64), (width, height));
    });
}
