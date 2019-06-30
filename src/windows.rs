//! Standard Windows functions

use std::f32::consts::PI;
use crate::Frame;
use crate::vectors;


/// A window function. Can be applied to a signal
#[derive(Clone, Debug, PartialEq)]
pub struct Window {
    samples: Vec<f32>,
}

impl Window {
    /// Returns the length of the window, in frames
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Returns a `Vector` without comsuming this window.
    pub fn to_vec(&self) -> &Vec<f32> {
        &self.samples
    }

    /// Apply this window to the given frame
    pub fn apply(&self, input: &Frame, mut output: &mut Frame) {
        vectors::multiply(&self.samples, &input, &mut output);
    }

    /// Apply this window to the given signal centering the window in the
    /// position specified. Works even when window and signal have different lengths
    pub fn apply_with_center(&self, _input: &Frame, _center: usize, _output: &mut Frame) {
        // // If the window is applied such that it does not intersect the signal,
        // // or at least one between the signal and the windows has zero length,
        // // we return an empty signal immediately
        // let maxr = center + (self.len() / 2) as isize;
        // if maxr < 0 || self.len() == 0 || signal.len() == 0 {
        //     return Signal::from_samples(vec![], signal.sample_rate());
        // }

        // // Compute the length of the resulting signal. It is the minimum between
        // // the length of the source signal and the right end of the translated window.
        // // Also compute the right end of the portion of window to consider in the multiplication
        // let mut rs = maxr as usize;
        // let mut rw = self.len();
        // if rs > signal.len() {
        //     rw = self.len() - (rs - signal.len());
        //     rs = signal.len();
        // }
        // let mut samples = Vector::with_capacity(rs);
        // // Compute the left end of the singal and of the portion of window to consider
        // let mut ls = center - (self.len() / 2) as isize;
        // let lw;
        // if ls > 0 {
        //     lw = 0;
        //     // If the signal to consider starts after the 0, fill the first part of the result
        //     // with zeros
        //     for _ in 0..ls {
        //         samples.push(Complex::new(0.0, 0.0));
        //     }
        // } else {
        //     lw = -ls;
        //     ls = 0;
        // }
        // let (ls, lw) = (ls as usize, lw as usize);

        // // FIXME: using a good linalg crate it should be possible to use less allocations here
        // samples.append(&mut signal.to_vec()[ls..rs].multiply(&self.to_vec()[lw..rw].to_vec()));
        // Signal::from_samples(samples, signal.sample_rate())
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
                1.0 - ((x as f32 - (frame_length - 1) as f32 / 2.0)
                    / ((frame_length - 1) as f32 / 2.0))
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
                1.0 - ((x as f32 - (frame_length - 1) as f32 / 2.0)
                    / ((frame_length - 1) as f32 / 2.0))
                    .powi(2)
            })
            .collect(),
    }
}

pub fn sine(frame_length: usize) -> Window {
    Window {
        samples: (0..frame_length)
            .map(|x| (PI * x as f32 / (frame_length - 1) as f32).sin())
            .collect(),
    }
}

pub fn hann(frame_length: usize) -> Window {
    Window {
        samples: (0..frame_length)
            .map(|x| 0.5 * (1.0 - (2.0 * PI * x as f32 / (frame_length - 1) as f32).cos()))
            .collect(),
    }
}

/// Compute a hamming window of the given size
pub fn hamming(frame_length: usize) -> Window {
    let a0 = 25.0 / 46.0;

    Window {
        samples: (0..frame_length)
            .map(|x| a0 - (1.0 - a0) * ((2.0 * PI * x as f32 / (frame_length - 1) as f32).cos()))
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
                a0 - a1 * (2.0 * PI * x as f32 / (frame_length - 1) as f32).cos()
                    + a2 * (4.0 * PI * x as f32 / (frame_length - 1) as f32).cos()
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

    #[test]
    fn test_base_rectangular() {
        let w = rectangular(10);
        assert_eq!(w.len(), 10);
        assert_eq!(w.to_vec(), &vec![1.0f32; 10]);
    }

    #[test]
    fn test_apply() {
        let w = triangular(10);
        let frame: Frame = vec![1.0; 10];
        let mut output: Frame = vec![0.0; 10];

        w.apply(&frame, &mut output);
        assert_eq!(w.to_vec(), &output);
    }

    // #[test]
    // fn test_apply_with_center() {
    //     let w = triangular(11);
    //     let frame: Frame = vec![1.0; 20];
    //     let mut output: Frame = vec![0.0; 20];

    //     w.apply_with_center(&frame, 10, &mut output);

    //     assert_eq!(0.0, output[2]);
    //     assert_eq!(1.0, output[10]);
    // }
}
