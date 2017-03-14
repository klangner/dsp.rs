extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::signal::*;


fn main() {
    let xs = sample(&chirp(1.0, 20.0, 3.0), (0..500).map(|x| (x as f64)*3.0/500.0).collect());

    let idx: Vec<usize> = (0..xs.len()).collect();
    let ys: Vec<f64> = xs.iter().map(|x| x.re).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ys, &[Color("red")]);
    fg.show();
}