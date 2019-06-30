//! # Digital Signal Processing
//!
//! Signals can be processed in Time or Frequency domain
//!

pub mod generators;
pub mod fft;
pub mod spectrums;
mod vectors;
pub mod windows;
// Remove this
mod vectors_old;

use num_complex::Complex32;


/// Time domain data buffer
pub type Frame = Vec<f32>;

/// Frequency domain data buffer based on complex numbers
pub type Spectrum = Vec<Complex32>;


/// This trait is implemented by node which is used to generate signals
pub trait SourceNode {
    /// Generate next batch of data samples.
    /// Data is generate into provided buffer
    /// Return number of generated samples
    fn next(&mut self, output: &mut Frame) -> usize;
}


/// This trait is implemented by node which process signals
pub trait ProcessingNode {
    /// Generate next batch of data samples based on input data
    /// Return number of processed samples
    fn process(&mut self, input: &Frame, output: &mut Frame) -> usize;
}


/// This trait is implemented by node which consumes signal.
pub trait DestinationNode {
    /// Consume input data.
    /// Return number of consumed samples
    fn consume(&mut self, input: &Frame) -> usize;
}
