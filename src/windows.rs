//! Standard Windows functions

use num_complex::Complex;
use vectors::{Vector, VectorImpl};

/// A window function. Can be applied to a signal
#[derive(Clone)]
pub struct Window {
    samples: Vec<f64>,
}

impl Window {
    pub fn into_vector(self) -> Vector {
        self.samples
            .into_iter()
            .map(|x| Complex::new(x, 0.0))
            .collect()
    }
}

/// Compute a simple rectangular window, a.k.a. __boxcar__ or __Dirichlet__ window
pub fn rectangular(frame_length: usize) -> Window {
    Window {
        samples: vec![1.0; frame_length],
    }
}

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

/// Compute a hamming window of the given size
pub fn hamming(frame_length: usize) -> Window {
    use std::f64::consts::PI;
    let a0 = 25.0 / 46.0;

    Window {
        samples: (0..frame_length)
            .map(|x| a0 - (1.0 - a0) * ((2.0 * PI * x as f64 / (frame_length - 1) as f64).cos()))
            .collect(),
    }
}
