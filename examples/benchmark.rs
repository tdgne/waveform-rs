extern crate waveform;

use waveform::*;
use std::time::SystemTime;

fn main() {
    let ratio = 1.0f64; // The ratio of samples to render.
    let width = 1000usize; // The width of the rendered image.
    let bin_size = 10usize; // Bin size for BinnedWaveformRenderer
    let multi_bin_sizes: Vec<usize> = vec![10,50,100,500,1000]; // Bin sizes for MultiWaveformRenderer

    let mut samples: Vec<f64> = Vec::new();
    for t in 0..441000 {
        // 10 seconds
        samples.push(
            ((t as f64) / 100f64 * 2f64 * 3.1415f64).sin() *
                ((t as f64) / 10000f64 * 2f64 * 3.1415f64).sin(),
        );
    }

    let config = WaveformConfig::new(
        -1f64, 1f64,
        Color::RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
        Color::RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    ).unwrap();

    // Multilevel binned version
    {
        let mut wfg = MultiWaveformRenderer::new(
            &SampleSequence {
                data: &samples[..],
                sample_rate: 44100f64,
            },
            &multi_bin_sizes,
            config.clone(),
        ).unwrap();
        let now = SystemTime::now();
        for _ in 0..100 {
            wfg.render_vec(TimeRange::Seconds(0f64, 10f64*ratio), (width, 100));
        }
        if let Ok(elapsed) = now.elapsed() {
            println!("Multi * 1000 times:  {} secs + {} nsecs", elapsed.as_secs(), elapsed.subsec_nanos());
        }
    }

    // Binned version
    {
        let mut wfg = BinnedWaveformRenderer::new(
            &SampleSequence {
                data: &samples[..],
                sample_rate: 44100f64,
            },
            bin_size,
            config.clone(),
        ).unwrap();
        let now = SystemTime::now();
        for _ in 0..100 {
            wfg.render_vec(TimeRange::Seconds(0f64, 10f64*ratio), (width, 100));
        }
        if let Ok(elapsed) = now.elapsed() {
            println!("Binned * 1000 times: {} secs + {} nsecs", elapsed.as_secs(), elapsed.subsec_nanos());
        }
    }
}
