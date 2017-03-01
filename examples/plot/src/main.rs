extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::signals::{SignalImpl};
use dsp::gen::*;


fn main() {
//    let xs = impulse(1024).shift(200);
    let xs = step(1024).shift(200);

    let idx: Vec<usize> = (0..xs.len()).collect();
    let ys: Vec<f32> = xs.iter().map(|x| x.re).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ys, &[Color("red")]);
    fg.show();
}