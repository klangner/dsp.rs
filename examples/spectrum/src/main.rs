extern crate gnuplot;
extern crate num_complex;
extern crate dsp;

use gnuplot::{Figure, Color, Caption};
use dsp::vectors::{Vector};
use dsp::signal::*;
use dsp::freq::{FourierTransform};


const SAMPLE_RATE: usize = 44100;
const SAMPLE_SIZE: usize = 2048;


fn main() {
    let signal = chirp(110.0, 880.0, 1.0);
    let mut ft = FourierTransform::forward(SAMPLE_RATE, SAMPLE_SIZE);

    // Calculate FFT every 1/10th of the second
    let ps: Vec<f64> = (0..100).map(|i| {
        let start_pos = (i as f64)/100.0;
        let range = (0..SAMPLE_SIZE).map(|x| start_pos+(x as f64)/(SAMPLE_RATE as f64)).collect();
        let xs = Vector::new(sample(&signal, range));
        let spectrum = ft.process(&xs);
        let argmax = spectrum.argmax();
        if argmax < SAMPLE_SIZE/2 {ft.item_freq(argmax)} else {ft.item_freq(SAMPLE_SIZE-argmax)}
    }).collect();

    let idx: Vec<f64> = (0..ps.len()).map(|x| (x as f64)/100.0).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ps, &[Color("blue"), Caption("Frequency in Hz.")]);
    fg.show();
}

//fn main() {
//    let signal = chirp(110.0, 880.0, 1.0);
//    let mut ft = FourierTransform::new(SAMPLE_RATE, SAMPLE_SIZE);
//
//    let range = (0..SAMPLE_SIZE).map(|x| 0.5+(x as f64)/(SAMPLE_RATE as f64)).collect();
//    let xs = Vector::new(sample(&signal, range));
//    let spectrum = ft.forward(&xs);
//    let powers: Vec<f64> = spectrum.to_vec().iter().map(|x| x.norm()).collect();
//    println!("Max at: {}", ft.item_freq(spectrum.argmax()));
//    let idx: Vec<f64> = (0..spectrum.len()/10).map(|x| ft.item_freq(x)).collect();
//    let mut fg = Figure::new();
//    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
//    fg.show();
//}