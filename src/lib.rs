//! # Digital Signal Processing
//!
//! Signals can be processed in Time or Frequency domain
//!

pub mod generators;
pub mod fft;
pub mod spectrums;
mod vectors;
pub mod windows;

use num_complex::Complex32;


/// Time domain data buffer. Uses Real number
pub type RealBuffer = Vec<f32>;

/// Frequency domain data buffer based on complex numbers
pub type ComplexBuffer = Vec<Complex32>;


/// This trait is implemented by node which is used to generate signals
pub trait SourceNode {
    /// Generate next batch of data samples.
    /// Data is generate into provided buffer
    /// Return number of generated samples
    fn next(&mut self, output: &mut RealBuffer) -> usize;

    /// Function for checking if generator has next frame of data
    /// Return true if it has.
    fn has_next(&self) -> bool { true }
}


/// This trait is implemented by node which process signals
pub trait ProcessingNode {
    /// Generate next batch of data samples based on input data
    /// Return number of processed samples
    fn process(&mut self, input: &RealBuffer, output: &mut RealBuffer) -> usize;
}


/// This trait is implemented by node which consumes signal.
pub trait DestinationNode {
    /// Consume input data.
    /// Return number of consumed samples
    fn consume(&mut self, input: &RealBuffer) -> usize;
}
