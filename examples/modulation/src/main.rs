extern crate gnuplot;
extern crate dsp;

use gnuplot::{Figure, Color, Caption};
use dsp::vectors::{Vector};
use dsp::signal::{cosine, sample, modulate};
use dsp::freq::{fft};

// Dimension
static N: usize = 256;


fn main() {
    // Our testing signal has 4Hz
    let signal = cosine(10./(N as f64), 0.);
    let ds = Vector::new(sample(&signal, 0.0, N as f64, 1.));
    // Our carrier signal has 50Hz
    let carrier = cosine(50./(N as f64), 0.);
    let cs = Vector::new(sample(&carrier, 0.0, N as f64, 1.));
    let modulated = modulate(signal, carrier);
    let ms = Vector::new(sample(&modulated, 0.0, N as f64, 1.));

    plot(&ds, "Source signal");
    plot(&cs, "Carrier");
    plot(&ms, "Modulated signal");
}

fn plot(xs: &Vector, title: &str) {
    let spectrum = fft(&xs);
    let powers: Vec<f64> = spectrum.to_vec().iter().map(|x| x.re).collect();
    let idx: Vec<usize> = (0..xs.len()).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, powers, &[Color("red"), Caption(title)]);
    fg.show();
}