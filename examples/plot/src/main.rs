extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use num_complex::{Complex};
use dsp::signals::{signal};


fn main() {
    let xs = signal(vec![Complex::new(1., 2.),
                         Complex::new(12., 4.),
                         Complex::new(7., 6.),
                         Complex::new(14., 8.)]);

    let idx: Vec<usize> = (0..xs.len()).collect();
    let ys: Vec<f32> = xs.iter().map(|x| x.re).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ys, &[Color("red")]);
    fg.show();
}