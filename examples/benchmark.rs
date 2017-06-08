extern crate waveform;

use waveform::*;

use std::time::{Duration, SystemTime};

fn main() {
    let mut samples: Vec<f64> = Vec::new();
    for t in 0..441000 {  // 10 seconds
        samples.push(
            ((t as f64) / 100f64 * 2f64 * 3.1415f64).sin()
            * ((t as f64) / 10000f64 * 2f64 * 3.1415f64).sin()
            );
    }

    // Cached version
    {
        let mut wfg = CachedWaveformGenerator::new(samples.clone(), 44100f64, 1f64, -1f64, Color{r:0,g:0,b:0,a:255}, Color{r:0,g:0,b:0,a:0});
        let now = SystemTime::now();
        for _ in 0..100 {
            wfg.generate_vec((0f64, 10f64), (1000, 100));
        }
        if let Ok(elapsed) = now.elapsed() {
            println!("{} {}", elapsed.as_secs(), elapsed.subsec_nanos());
        }
    }

    // Naive version
    {
        let wfg = SimpleWaveformGenerator{sample_rate: 44100f64, amp_max: 1f64, amp_min: -1f64,
                                            foreground: Color{r:0,g:0,b:0,a:255}, background: Color{r:0,g:0,b:0,a:0}};
        let now = SystemTime::now();
        for _ in 0..100 {
            wfg.generate_vec(&samples, (1000, 100));
        }
        if let Ok(elapsed) = now.elapsed() {
            println!("{} {}", elapsed.as_secs(), elapsed.subsec_nanos());
        }
    }
}
