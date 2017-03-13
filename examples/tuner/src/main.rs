extern crate gnuplot;
extern crate num_complex;
extern crate audrey;
extern crate dsp;


use gnuplot::{Figure, Color};
use dsp::vectors::{Vector};
use dsp::freq::{FourierTransform};

const SAMPLE_RATE: f64 = 44100.0;
const SAMPLE_SIZE: usize = 4096;


fn main() {
    let mut ft = FourierTransform::new(SAMPLE_RATE as usize, SAMPLE_SIZE);
    let mut reader = audrey::open("../sounds/sine_440hz.wav").unwrap();
    let mut samples: Vec<f32> = reader.samples().map(Result::unwrap).collect();

    samples.truncate(SAMPLE_SIZE);
    let xs = Vector::from_reals(samples.iter().map(|&x| x as f64).collect());
    let spectrum = ft.forward(&xs);

    // Print estimated frequency
    let idx = spectrum.argmax();
    let freq = if idx < SAMPLE_SIZE / 2 {ft.item_freq(idx)} else {SAMPLE_RATE-ft.item_freq(idx)};
    println!("Freq: {}Hz", freq);

    // Plot
    let powers: Vec<f64> = spectrum.to_vec().iter().map(|x| x.norm()).collect();
    let idx: Vec<f64> = (0..powers.len()/2).map(|x| (x as f64)* SAMPLE_RATE/(SAMPLE_SIZE as f64)).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.show();
}
