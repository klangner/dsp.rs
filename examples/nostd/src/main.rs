#![no_std]

extern crate num_complex;
extern crate dsp;

use dsp::vectors::{Vector};
use dsp::signal::{cosine, sample};
use dsp::freq::{fft};

// Dimension
static N: usize = 64;


fn main() {
    let signal = cosine(4./(N as f64), 0.);
    let xs = Vector::new(sample(&signal, 0.0, N as f64, 1.));
    let spectrum = fft(&xs);
    let powers: Vec<f64> = spectrum.to_vec().iter().map(|x| x.re).collect();
}
