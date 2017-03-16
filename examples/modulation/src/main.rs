extern crate gnuplot;
extern crate dsp;

use gnuplot::{Figure, Color, Caption};
use dsp::vectors::{Vector};
use dsp::csignal::{cosine, sample, modulate};
use dsp::freq::{FourierTransform};

const N: usize = 4096;


fn main() {
    // Our testing signal has 1Hz
    let signal = cosine(2./(N as f64), 0.);
    let ds = Vector::new(sample(&signal, (0..N).map(|x| x as f64).collect()));
    // Our carrier signal has 20Hz
    let carrier = cosine(20./(N as f64), 0.);
    let cs = Vector::new(sample(&carrier, (0..N).map(|x| x as f64).collect()));
    // Modulated signal
    let modulated = modulate(signal, carrier);
    let ms = Vector::new(sample(&modulated, (0..N).map(|x| x as f64).collect()));

    time_plot(&ms);
    freq_plot(&ds, &cs, &ms);

    demodulated_plot(&ds, &cs, &ms);
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
    let mut ft = FourierTransform::forward(N, N);
    let spectrum = ft.process(&v);
    spectrum.to_vec().iter().map(|x| x.re).collect()
}

fn demodulated_plot(source: &Vector, carrier: &Vector, modulated: &Vector) {
    let mut ft = FourierTransform::forward(N, N);
    let mut fti = FourierTransform::inverse(N, N);
    let demodulated = modulated.multiply(carrier);
    // Convert to freq domain to apply filter
    let ps = ft.process(&demodulated);
    let ds = fti.process(&ps);
    let xs: Vec<f64> = source.to_vec().iter().map(|x| x.re).collect();
    let xs2: Vec<f64> = ds.to_vec().iter().map(|x| x.re/(N as f64)).collect();
    let idx: Vec<usize> = (0..N).collect();
    let mut fg = Figure::new();
    fg.set_terminal("x11 size 800, 400","");
    fg.axes2d()
        .lines(&idx, xs, &[Color("green"), Caption("Source signal")])
        .lines(&idx, xs2, &[Color("red"), Caption("Demodulated signal")]);
    fg.show();

}

