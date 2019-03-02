//! # Digital Signal Processing
//!
//! Signals can be processed in Time or Frequency domain
//!

extern crate num_complex;
extern crate rand;
extern crate rustfft;

pub mod fft;
pub mod generators;
pub mod signals;
pub mod spectrums;
mod vectors;
pub mod windows;
