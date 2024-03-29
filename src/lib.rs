//! # Digital Signal Processing
//!
//! Signals can be processed in Time or Frequency domain
//!

pub mod core;
pub mod filter;
pub mod signal;
pub mod spectrum;
pub mod window;
pub use num_complex;
#[cfg(feature = "audio")]
pub mod audio;

mod vector;


