extern crate waveform;
extern crate gtk;
extern crate gdk_pixbuf;

use waveform::*;
use gtk::{Window, WindowType, WindowExt, WidgetExt, Inhibit, Image, ContainerExt};
use gdk_pixbuf::Pixbuf;

fn main() {
    if gtk::init().is_err() {
        panic!("Failed to initialize gtk.");
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("A SimpleWaveformGenerator Test");
    window.set_default_size(800, 100);
    window.connect_delete_event(|_,_| {
        gtk::main_quit();
        Inhibit(false)
    });
    let mut samples: Vec<f64> = Vec::new();
    for t in 0..44100 {
        samples.push(
            ((t as f64) / 100f64 * 2f64 * 3.1415f64).sin()
            * ((t as f64) / 10000f64 * 2f64 * 3.1415f64).sin()
            );
    }

    let ss = SampleSequence{data: samples.clone(), sample_rate: 44100f64, range: TimeRange::Seconds(0f64, 1f64)};
    let config = WaveformConfig{foreground: Color{r:0,g:0,b:0,a:255}, background: Color{r:0,g:0,b:0,a:0}, amp_min: -1f64, amp_max: 1f64};
    let mut wfg = CachedWaveformGenerator::new(ss, config);
    let vec = wfg.generate_vec(TimeRange::Seconds(0f64, 1f64), (800, 100)).unwrap();
    let pixbuf = Pixbuf::new_from_vec(vec, 0, true, 8, 800, 100, 800*4);
    let image = Image::new_from_pixbuf(Some(&pixbuf));
    window.add(&image);
    window.show_all();
    gtk::main();
}
