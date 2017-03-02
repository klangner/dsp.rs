extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use num_complex::{Complex64};
use gnuplot::{Figure, Color};
use dsp::hilbert::*;
use dsp::signal::*;

// Dimension
static N: usize = 64;


fn main() {
    // Our testing signal has 4Hz
    let signal = cosine(4./(N as f64), 0.);
    let xs = vector(sample(&signal, 0.0, N as f64, 1.));
    let mut vs: Vec<Complex64> = Vec::with_capacity(N);

    for n in 0..N {
        let base = fourier_base(N, n);
        vs.push(xs.dot(&base));
    }

    let powers: Vec<f64> = vs.iter().map(|x| x.re).collect();
    plot(&powers);

    let xs_real: Vec<f64> = xs.iter().map(|x| x.re).collect();
    plot(&xs_real);

//    let bs: Vec<f64> = fourier_base(N, 4).iter().map(|x| x.re).collect();
//    plot(&xs_real);
}

fn plot(xs: &Vec<f64>) {
    let idx: Vec<usize> = (0..xs.len()).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, xs, &[Color("red")]);
    fg.show();

}