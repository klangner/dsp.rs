extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::signal::*;
use dsp::freq::{FourierTransform};


const SAMPLE_RATE: usize = 44100;
const SAMPLE_SIZE: usize = 1024;

fn main() {
    let mut ft = FourierTransform::new(SAMPLE_RATE, SAMPLE_SIZE);
    let xs = sample(&chirp(110.0, 880.0, 1.0), 0.0, 1.0, 1.0/(SAMPLE_RATE as f64));

    // Calculate FFT every 1/10th of the second
//    for i in (0.0..1.0).step_by(0.1) {
//        let spectrum = ft.forward(&xs);
//    }
    println!("Not implemented yet!");

//    let idx: Vec<usize> = (0..xs.len()).collect();
//    let ys: Vec<f64> = xs.iter().map(|x| x.re).collect();
//    let mut fg = Figure::new();
//    fg.axes2d().lines(&idx, &ys, &[Color("red")]);
//    fg.show();
}