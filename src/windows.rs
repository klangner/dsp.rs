//! Standard Windows functions

use num_complex::Complex;
use signals::Signal;
use vectors::{Vector, VectorImpl};

/// A window function. Can be applied to a signal
#[derive(Clone)]
pub struct Window {
    samples: Vec<f64>,
}

impl Window {
    /// returns a `Vector` comsuming this window.
    pub fn into_vector(self) -> Vector {
        self.samples
            .into_iter()
            .map(|x| Complex::new(x, 0.0))
            .collect()
    }

    /// returns a `Vector` without comsuming this window.
    pub fn to_vec(&self) -> Vector {
        self.samples
            .iter()
            .map(|x| Complex::new(x.clone(), 0.0))
            .collect()
    }

    /// apply this window to the given signal
    pub fn apply(&self, signal: Signal) -> Signal {
        assert_eq!(self.samples.len(), signal.len());
        Signal::from_samples(
            signal.to_vec().multiply(&(self.to_vec())),
            signal.sample_rate(),
        )
    }
}

/// Compute a simple rectangular window, a.k.a. __boxcar__ or __Dirichlet__ window
pub fn rectangular(frame_length: usize) -> Window {
    Window {
        samples: vec![1.0; frame_length],
    }
}

/// Creates a triangular window
pub fn triangular(frame_length: usize) -> Window {
    Window {
        samples: (0..frame_length)
            .map(|x| {
                1.0 - ((x as f64 - (frame_length - 1) as f64 / 2.0)
                    / ((frame_length - 1) as f64 / 2.0))
                    .abs()
            }).collect(),
    }
}

/// Create the Welch window
pub fn welch(frame_length: usize) -> Window {
    Window {
        samples: (0..frame_length)
            .map(|x| {
                1.0 - ((x as f64 - (frame_length - 1) as f64 / 2.0)
                    / ((frame_length - 1) as f64 / 2.0))
                    .powi(2)
            }).collect(),
    }
}
use std::f64::consts::PI;

pub fn sine(frame_length: usize) -> Window {
    Window {
        samples: (0..frame_length)
            .map(|x| (PI * x as f64 / (frame_length - 1) as f64).sin())
            .collect(),
    }
}

pub fn hann(frame_length: usize) -> Window {
    Window {
        samples: (0..frame_length)
            .map(|x| 0.5 * (1.0 - (2.0 * PI * x as f64 / (frame_length - 1) as f64).cos()))
            .collect(),
    }
}

/// Compute a hamming window of the given size
pub fn hamming(frame_length: usize) -> Window {
    let a0 = 25.0 / 46.0;

    Window {
        samples: (0..frame_length)
            .map(|x| a0 - (1.0 - a0) * ((2.0 * PI * x as f64 / (frame_length - 1) as f64).cos()))
            .collect(),
    }
}

pub fn blackman(frame_length: usize) -> Window {
    let a0 = 7938.0 / 18608.0;
    let a1 = 9240.0 / 18608.0;
    let a2 = 1430.0 / 18608.0;

    Window {
        samples: (0..frame_length)
            .map(|x| {
                a0 - a1 * (2.0 * PI * x as f64 / (frame_length - 1) as f64).cos()
                    + a2 * (4.0 * PI * x as f64 / (frame_length - 1) as f64).cos()
            }).collect(),
    }
}
