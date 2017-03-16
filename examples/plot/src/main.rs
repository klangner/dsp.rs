extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::vectors::{Vector};
use dsp::time::{SpatialSignal};
use dsp::csignal::*;


const N: usize = 1000;


fn main() {
//    let xs = sample(&chirp(1.0, 20.0, 3.0), (0..500).map(|x| (x as f64)*3.0/500.0).collect());
    let xs = sample(&complex(3.0, 0.0), (0..N).map(|x| (x as f64)/(N as f64)).collect());
    let xs2: Vector = Vector::new(xs).add_noise(0.05);

    let idx: Vec<usize> = (0..xs2.len()).collect();
    let ys: Vec<f64> = xs2.iter().map(|x| x.re).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ys, &[Color("red")]);
    fg.show();
}