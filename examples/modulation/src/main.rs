extern crate gnuplot;
extern crate dsp;

use gnuplot::{Figure, Color, Caption};
use dsp::vectors::{Vector};
use dsp::signal::{cosine, sample, modulate};
use dsp::freq::{FourierTransform};

const N: usize = 4096;


fn main() {
    // Our testing signal has 1Hz
    let signal = cosine(1./(N as f64), 0.);
    let ds = Vector::new(sample(&signal, (0..N).map(|x| x as f64).collect()));
    // Our carrier signal has 20Hz
    let carrier = cosine(20./(N as f64), 0.);
    let cs = Vector::new(sample(&carrier, (0..N).map(|x| x as f64).collect()));
    // Modulated signal
    let modulated = modulate(signal, carrier);
    let ms = Vector::new(sample(&modulated, (0..N).map(|x| x as f64).collect()));

    time_plot(&ms);
    freq_plot(&ds, &cs, &ms);
}

fn time_plot(vector: &Vector) {
    let xs: Vec<f64> = vector.to_vec().iter().map(|x| x.re).collect();
    let idx: Vec<usize> = (0..N).collect();
    let mut fg = Figure::new();
    fg.set_terminal("x11 size 800, 400","");
    fg.axes2d().lines(&idx, &xs, &[Color("green"), Caption("Modulated signal")]);
    fg.show();

}

fn freq_plot(ds: &Vector, cs: &Vector, ms: &Vector) {
    let idx: Vec<usize> = (0..40).collect();
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&idx, powers(ds), &[Color("green"), Caption("Source signal")])
        .lines(&idx, powers(cs), &[Color("blue"), Caption("Carrier signal")])
        .lines(&idx, powers(ms), &[Color("red"), Caption("Modulated signal")]);
    fg.show();
}


fn powers(v: &Vector) -> Vec<f64> {
    let mut ft = FourierTransform::new(N, N);
    let spectrum = ft.forward(&v);
    spectrum.to_vec().iter().map(|x| x.re).collect()
}