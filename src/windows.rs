//! Standard Windows functions

use num_complex::Complex;
use vectors::{Vector, VectorImpl};

/// A window function. Can be applied to a signal
#[derive(Clone)]
pub struct Window<T>
where
    T: Fn(f64) -> Complex<f64>,
{
    gen: T,
    len: usize,
}

impl<T> Window<T>
where 
      T: Fn(f64) -> Complex<f64>,
{
    pub fn into_vec(self) -> Vector {
        (0..self.len).
            map(|x| (self.gen)(x as f64))
            .collect()
    }
}

/// Compute a simple rectangular window, a.k.a. __boxcar__ or __Dirichlet__ window
pub fn rectangular(frame_length: usize) -> Window<impl Fn(f64) -> Complex<f64>> {
    Window {
        gen: move |x| {
            if x >= 0. && x < frame_length as f64 {
                Complex::new(1.0, 0.0)
            } else {
                Complex::new(0.0, 0.0)
            }
        },
        len: frame_length,
    }
}

pub fn triangular(frame_length: usize) -> Window<impl Fn(f64) -> Complex<f64>> {
    Window {
        gen: move |x| {
            if x >= 0. && x < frame_length as f64 {
                Complex::new(
                    1.0 - ((x - (frame_length - 1) as f64 / 2.0)
                        / ((frame_length - 1) as f64 / 2.0))
                        .abs(),
                    0.0,
                )
            } else {
                Complex::new(0.0, 0.0)
            }
        },
        len: frame_length,
    }
}

/// Compute a hamming window of the given size
pub fn hamming(frame_length: usize) -> Window<impl Fn(f64) -> Complex<f64>> {
    use std::f64::consts::PI;
    let a0 = 25.0 / 46.0;

    Window {
        gen: move |x| {
            if x >= 0.0 && x < frame_length as f64 {
                Complex::new(
                    a0 - (1.0 - a0) * ((2.0 * PI * x as f64 / (frame_length - 1) as f64).cos()),
                    0.0,
                )
            } else {
                Complex::new(0.0, 0.0)
            }
        },
        len: frame_length,
    }
}
