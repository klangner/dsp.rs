extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color};
use dsp::vectors::{Vector};
use dsp::signal::*;
use dsp::freq::{FourierTransform};


const SAMPLE_RATE: usize = 44100;
const SAMPLE_SIZE: usize = 2048;

fn main() {
    let signal = chirp(110.0, 880.0, 1.0);
    let mut ft = FourierTransform::new(SAMPLE_RATE, SAMPLE_SIZE);

    // Calculate FFT every 1/10th of the second
    let ss: Vec<f64> = (0..100).map(|i| {
        let step = (SAMPLE_SIZE as f64)/(SAMPLE_RATE as f64);
        let idx = ((i*SAMPLE_RATE) as f64)/100.0;
        let xs = Vector::new(sample(&signal, (0..SAMPLE_SIZE).map(|x| (x as f64)).collect()));
        let spectrum = ft.forward(&xs);
        let argmax = spectrum.argmax();
        ft.item_freq(argmax)
    }).collect();

//    let idx: Vec<usize> = (0..xs.len()).collect();
//    let ys: Vec<f64> = xs.iter().map(|x| x.re).collect();
//    let mut fg = Figure::new();
//    fg.axes2d().lines(&idx, &ys, &[Color("red")]);
//    fg.show();
}