// The GUI parts conditioned by the "example-gui" feature
// depend on the gtk crate, which is an interface to the 
// native GTK libs.
// Although, the other parts which are not conditioned
// altogether demonstrate the basic use of this crate.

extern crate waveform;

#[cfg(feature = "example-gui")]
extern crate gtk;
#[cfg(feature = "example-gui")]
extern crate gdk_pixbuf;

use waveform::{
    SampleSequence,
    WaveformConfig,
    Color,
    BinnedWaveformRenderer,
    TimeRange,
};

#[cfg(feature = "example-gui")]
use gtk::{ContainerExt, Image, Inhibit, WidgetExt, Window, WindowExt, WindowType};
#[cfg(feature = "example-gui")]
use gdk_pixbuf::Pixbuf;

fn main() {
    #[cfg(feature = "example-gui")]
    {
        if gtk::init().is_err() {
            panic!("Failed to initialize gtk.");
        }
    }

    #[cfg(feature = "example-gui")]
    let window = Window::new(WindowType::Toplevel);
    #[cfg(feature = "example-gui")]
    {
        window.set_title("A simple waveform renderer test");
        window.set_default_size(800, 100);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }

    // Generate samples to show.
    let mut samples: Vec<f64> = Vec::new();
    for t in 0..44100 {
        samples.push(
            ((t as f64) / 100f64 * 2f64 * 3.1415f64).sin() * ((t as f64) / 10000f64 * 2f64 * 3.1415f64).sin(),
        );
    }

    // The renderer's config.
    let config = WaveformConfig::new(
        -1f64, // Minimum amplitude to show
        1f64, // Maximum amplitude to show

        // Foreground color
        Color::Vector4(0, 0, 0, 255),

        // Background color
        Color::Vector4(0, 0, 0, 0)
    ).unwrap();

    // Put a reference to the samples here along with its sample rate.
    // We need to set a sample rate because it will be used
    // when you specify the time range in seconds.
    let ss = SampleSequence {
        data: &samples[..],
        sample_rate: 44100f64,
    };

    // Construct the renderer.
    // The second argument is the bin size.
    // The renderer will bin the samples, and then
    // calculate the minimum and maximum amplitude values for
    // each bin.
    let wfg = BinnedWaveformRenderer::new(&ss, 10, config).unwrap();

    // Render!
    // The renderer doesn't look at the actual audio samples here.
    // Instead it will use the binned min/max values calculated above,
    // making the rendering quite faster.
    let vec: Vec<u8> =
        wfg.render_vec(TimeRange::Seconds(0.0f64, 1.0f64), (800, 100))
        .unwrap();

    #[cfg(feature = "example-gui")]
    {
        let pixbuf =
            Pixbuf::new_from_vec(vec, 0, true, 8, 800, 100, 800 * 4);
        let image = Image::new_from_pixbuf(Some(&pixbuf));
        window.add(&image);
        window.show_all();
        gtk::main();
    }
}
