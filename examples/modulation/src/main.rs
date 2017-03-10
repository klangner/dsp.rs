extern crate gnuplot;
extern crate dsp;

use gnuplot::{Figure, Color, Caption};
use dsp::vectors::{Vector};
use dsp::signal::{cosine, sample, modulate, Signal};
use dsp::freq::{fft};

// Dimension
static N: usize = 256;


fn main() {
    // Our testing signal has 4Hz
    let signal = cosine(10./(N as f64), 0.);
    let ds = powers(&signal);
    // Our carrier signal has 50Hz
    let carrier = cosine(50./(N as f64), 0.);
    let cs = powers(&carrier);
    // Modulated signal
    let modulated = modulate(signal, carrier);
    let ms = powers(&modulated);

    let idx: Vec<usize> = (0..N/2).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&idx, ds, &[Color("green"), Caption("Source signal")])
        .lines(&idx, cs, &[Color("blue"), Caption("Carrier signal")])
        .lines(&idx, ms, &[Color("red"), Caption("Modulated signal")]);
    fg.show();
}

fn powers(signal: &Signal) -> Vec<f64> {
    let ds = Vector::new(sample(&signal, 0.0, N as f64, 1.));
    let spectrum = fft(&ds);
    spectrum.to_vec().iter().map(|x| x.re).collect()
}