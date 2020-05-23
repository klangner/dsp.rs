//! # Digital Signal Processing
//!
//! Signals can be processed in Time or Frequency domain
//!

pub mod generators;
pub mod filters;
pub mod fft;
pub mod spectrums;
pub mod windows;
mod vectors;

pub use num_complex;
use crate::num_complex::Complex32;


/// Time domain data buffer. Uses Real number
pub type RealBuffer = Vec<f32>;

/// Frequency domain data buffer based on complex numbers
pub type ComplexBuffer = Vec<Complex32>;

/// Signal consists of:
/// * data
/// * sample_rate which is number of samples per second (Sampling frequency)
pub struct Signal {
    pub data: Vec<f32>,
    pub sample_rate: usize
}

impl Signal {

    /// create new signal with a given length and sample rate
    pub fn new(data: Vec<f32>, sample_rate: usize) -> Signal {
        Signal { data, sample_rate }
    }

    /// Length of the signal buffer
    pub fn len(&self) -> usize {
        self.data.len()
    }

}