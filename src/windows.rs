//! Standard Windows functions

use num_complex::Complex;
use signals::Signal;
use vectors::{Vector, VectorImpl};

/// A window function. Can be applied to a signal
#[derive(Clone, Debug, PartialEq)]
pub struct Window {
    samples: Vec<f64>,
}

impl Window {
    /// Returns the length of the window, in frames
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns a `Vector` comsuming this window.
    pub fn into_vector(self) -> Vector {
        self.samples
            .into_iter()
            .map(|x| Complex::new(x, 0.0))
            .collect()
    }

    /// Returns a `Vector` without comsuming this window.
    pub fn to_vec(&self) -> Vector {
        self.samples
            .iter()
            .map(|x| Complex::new(x.clone(), 0.0))
            .collect()
    }

    /// Apply this window to the given signal
    ///
    /// # Panics
    ///
    /// Panics if the signal and the window have different lengths.
    ///
    /// # Returns
    ///
    /// A new `Signal` obtained windowing the original.
    pub fn apply(&self, signal: &Signal) -> Signal {
        assert_eq!(
            self.samples.len(),
            signal.len(),
            "Signal and window should have the same length"
        );
        Signal::from_samples(
            signal.to_vec().multiply(&(self.to_vec())),
            signal.sample_rate(),
        )
    }

    /// Apply this window to the given signal centering the window in the
    /// position specified. Works even when window and signal have different lengths
    pub fn apply_with_center(&self, signal: &Signal, center: isize) -> Signal {
        // If the window is applied such that it does not intersect the signal,
        // or at least one between the signal and the windows has zero length,
        // we return an empty signal immediately
        let maxr = center + (self.len() / 2) as isize;
        if maxr < 0 || self.len() == 0 || signal.len() == 0 {
            return Signal::from_samples(vec![], signal.sample_rate());
        }

        // Compute the length of the resulting signal. It is the minimum between
        // the length of the source signal and the right end of the translated window.
        // Also compute the right end of the portion of window to consider in the multiplication
        let mut rs = maxr as usize;
        let mut rw = self.len();
        if rs > signal.len() {
            rw = self.len() - (rs - signal.len());
            rs = signal.len();
        }
        let mut samples = Vector::with_capacity(rs);
        // Compute the left end of the singal and of the portion of window to consider
        let mut ls = center - (self.len() / 2) as isize;
        let lw;
        if ls > 0 {
            lw = 0;
            // If the signal to consider starts after the 0, fill the first part of the result
            // with zeros
            for _ in 0..ls {
                samples.push(Complex::new(0.0, 0.0));
            }
        } else {
            lw = -ls;
            ls = 0;
        }
        let (ls, lw) = (ls as usize, lw as usize);

        // FIXME: using a good linalg crate it should be possible to use less allocations here
        samples.append(&mut signal.to_vec()[ls..rs].multiply(&self.to_vec()[lw..rw].to_vec()));
        Signal::from_samples(samples, signal.sample_rate())
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
            })
            .collect(),
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
            })
            .collect(),
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
            })
            .collect(),
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_base_rectangular() {
        let w = rectangular(10);
        assert_eq!(w.len(), 10);
        assert_eq!(
            w.to_vec(),
            (0..10).map(|_| Complex::new(1.0, 0.0)).collect::<Vec<_>>()
        );
    }

    use generators::step;
    #[test]
    fn test_apply() {
        let w = triangular(10);
        let s = step().generate((0..10).map(|i| i.into()).collect());

        assert_eq!(w.to_vec(), w.apply(&s).to_vec());
    }

    #[test]
    fn test_apply_with_center() {
        let w = triangular(11);
        let s = step().generate((0..20).map(|i| i.into()).collect());

        let new_signal = w.apply_with_center(&s, 10);

        assert_eq!(Complex::new(0.0, 0.0), new_signal.get(2));
        assert_eq!(Complex::new(1.0, 0.0), new_signal.get(10));
    }
}
