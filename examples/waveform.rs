extern crate waveform;
extern crate gtk;
extern crate gdk_pixbuf;

use waveform::*;
use waveform::direct::*;
use gtk::{ContainerExt, Image, Inhibit, WidgetExt, Window, WindowExt, WindowType};
use gdk_pixbuf::Pixbuf;

fn main() {
    // Whether to use BinnedWaveformRenderer or DirectWaveformRenderer
    let use_binned = true;

    if gtk::init().is_err() {
        panic!("Failed to initialize gtk.");
    }
    let window = Window::new(WindowType::Toplevel);
    window.set_title("A simple waveform renderer test");
    window.set_default_size(800, 100);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    let mut samples: Vec<f64> = Vec::new();
    for t in 0..44100 {
        samples.push(
            ((t as f64) / 100f64 * 2f64 * 3.1415f64).sin() * ((t as f64) / 10000f64 * 2f64 * 3.1415f64).sin(),
        );
    }

    let config = WaveformConfig::new(
        -1f64,
        1f64,
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
        },
    ).unwrap();

    let vec: Vec<u8>;
    if use_binned {
        let ss = SampleSequence {
            data: &samples[..],
            sample_rate: 44100f64,
        };
        let wfg = BinnedWaveformRenderer::new(&ss, 10, config).unwrap();
        vec = wfg.render_vec(TimeRange::Seconds(0.0f64, 1.0f64), (800, 100))
            .unwrap();
    } else {
        let wfg = DirectWaveformRenderer {
            sample_rate: 44100f64,
            config: config,
        };
        vec = wfg.render_vec(&samples, (800, 100)).unwrap();
    }

    let pixbuf = Pixbuf::new_from_vec(vec, 0, true, 8, 800, 100, 800 * 4);
    let image = Image::new_from_pixbuf(Some(&pixbuf));
    window.add(&image);
    window.show_all();
    gtk::main();
}
