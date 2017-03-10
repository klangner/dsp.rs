extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::signal::*;


fn main() {
    let xs = sample(&triangle(3.), -0.5, 0.5, 0.01);

    let idx: Vec<usize> = (0..xs.len()).collect();
    let ys: Vec<f64> = xs.iter().map(|x| x.re).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ys, &[Color("red")]);
    fg.show();
}