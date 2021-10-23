//! # Digital Signal Processing
//!
//! Signals can be processed in Time or Frequency domain
//!

pub mod node;
pub mod generator;
pub mod filter;
pub mod fft;
pub mod signal;
pub mod spectrum;
pub mod window;
mod vector;

pub use num_complex;
use crate::num_complex::Complex32;


/// Time domain data buffer. Uses Real number
pub type RealBuffer = Vec<f32>;

/// Frequency domain data buffer based on complex numbers
pub type ComplexBuffer = Vec<Complex32>;
