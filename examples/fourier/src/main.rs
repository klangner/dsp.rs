extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::vectors::{Vector};
use dsp::signal::{cosine, sample};
use dsp::freq::{FourierTransform};

// Dimension
static N: usize = 64;


fn main() {
    // Our testing signal has 4Hz
    let signal = cosine(4./(N as f64), 0.);
    let xs = Vector::new(sample(&signal, (0..N).map(|x| x as f64).collect()));
    let mut ft = FourierTransform::new(N, N);
    let spectrum = ft.forward(&xs);
//    println!("Max at {}: {}", spectrum);

    let powers: Vec<f64> = spectrum.to_vec().iter().map(|x| x.re).collect();
    plot(&powers);

    let xs_real: Vec<f64> = xs.to_vec().iter().map(|x| x.re).collect();
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