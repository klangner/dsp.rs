//! # Digital Signal Processing
//!
//! Signals can be processed in Time or Frequency domain
//!

pub mod fft;
pub mod generators;
pub mod signals;
pub mod spectrums;
mod vectors;
pub mod windows;

/// This trait is implemented by node which is used to generate signals
pub trait SourceNode {
    /// Generate next batch of data samples.
    /// Data is generate into provided buffer
    /// Return number of generated samples
    fn next(&mut self, output: &mut Vec<f32>) -> usize;
}


/// This trait is implemented by node which process signals
pub trait ProcessingNode {
    /// Generate next batch of data samples based on input data
    /// Return number of processed samples
    fn process(&mut self, input: &Vec<f32>, output: &mut Vec<f32>) -> usize;
}


/// This trait is implemented by node which consumes signal.
pub trait DestinationNode {
    /// Consume input data.
    /// Return number of consumed samples
    fn consume(&mut self, input: &Vec<f32>) -> usize;
}
