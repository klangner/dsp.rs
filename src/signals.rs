//! Process Discrete signals in time domain

use num_complex::{Complex, Complex64};
use std::cmp;
use vectors::{Vector, VectorImpl};
use windows::Window;

/// Discrete Time Signal
///   * data - Data points
///   * sample_rate - how many points per second
#[derive(Debug, PartialEq)]
pub struct Signal {
    pub sample_rate: usize,
    data: Vector,
}

impl Signal {
    /// Create new signal from 1 second of samples
    pub fn new(data: Vec<Complex64>) -> Signal {
        let n = data.len();
        Signal {
            data,
            sample_rate: n,
        }
    }

    /// Returns the sample_rate of this signal
    pub fn sample_rate(&self) -> usize {
        self.sample_rate
    }

    /// Create new signal from samples with given sample rate
    pub fn from_samples(data: Vec<Complex64>, sample_rate: usize) -> Signal {
        Signal { data, sample_rate }
    }

    /// Create new signal from vector of real numbers
    pub fn from_reals(data: Vec<f64>, sample_rate: usize) -> Signal {
        Signal {
            data: data.iter().map(|x| Complex::new(*x, 0.)).collect(),
            sample_rate,
        }
    }

    /// Signal length() in number of samples
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Signal duration in time units
    pub fn duration(&self) -> f64 {
        self.data.len() as f64 / self.sample_rate as f64
    }

    /// This function will return 0 if index out of bound
    pub fn get(&self, i: isize) -> Complex64 {
        let s = self.data.len() as isize;
        if i < 0 || i >= s {
            Complex::new(0., 0.)
        } else {
            self.data[i as usize]
        }
    }

    /// Copy data into new vector
    pub fn to_vec(&self) -> Vec<Complex64> {
        self.data.clone()
    }

    /// Shift signal by given integer
    /// y[n] = x[n-k]
    /// This function will not change the signal length
    pub fn shift(&self, k: isize) -> Signal {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.data.len());
        let size: isize = self.data.len() as isize;
        for n in 0..size {
            v.push(self.get(n - k));
        }
        Signal::new(v)
    }

    /// Integrate signal
    /// y[n] = Sum x[k] For all k <= n
    pub fn integrate(&self) -> Signal {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.data.len());
        let mut acc = Complex::new(0., 0.);
        for n in 0..self.data.len() {
            acc = acc + self.data.at(n);
            v.push(acc);
        }
        Signal::new(v)
    }

    /// Differentiate the signal
    /// y[n] = x[n] - x[n-1]
    pub fn differentiate(&self) -> Signal {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.data.len());
        let mut last = Complex::new(0., 0.);
        for n in 0..self.data.len() {
            v.push(self.data.at(n) - last);
            last = self.data.at(n);
        }
        Signal::new(v)
    }

    /// Calculate energy
    /// E = Sum x[n]^2 For all n
    pub fn energy(&self) -> f64 {
        self.data.iter().fold(0., |acc, &x| acc + (x * x.conj()).re)
    }

    /// Calculate power
    /// P = 1/N Sum x[n]^2 For all n
    pub fn power(&self) -> f64 {
        self.energy() / (self.data.len() as f64)
    }

    /// Modulate signal by given carrier
    pub fn modulate(&self, carrier: &Signal) -> Signal {
        let data = self.data.multiply(&carrier.data);
        Signal::new(data)
    }

    /// Sum 2 signals
    pub fn sum(&self, s2: &Signal) -> Signal {
        let data = self.data.add(&s2.data);
        Signal::new(data)
    }

    /// Sliced signal. Return copy.
    pub fn slice(&self, start: usize, end: usize) -> Signal {
        Signal::from_samples(self.data[start..end].to_vec(), self.sample_rate)
    }

    /// Reverse the signal. Last value will be first
    pub fn reverse(&self) -> Signal {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.data.len());
        let n = self.data.len();
        for i in 0..n {
            v.push(self.data.at(n - i - 1));
        }
        Signal::new(v)
    }

    /// Convolve signals
    pub fn convolve(&self, h: &Signal) -> Signal {
        let mut vs: Vec<Complex64> = Vec::with_capacity(self.data.len());
        for i in 0..self.len() {
            let mut v = Complex::new(0., 0.);
            for j in 0..cmp::min(i + 1, h.len()) {
                v = v + self.data.at(i - j) * h.data.at(j);
            }
            vs.push(v);
        }
        Signal::new(vs)
    }

    /// Build an iterator over frames of this signal.
    /// The frames have the specified length and spaced shift samples center to center
    pub fn frames<'a>(&'a self, length: usize, shift: usize) -> Frames<'a> {
        Frames {
            it: self.data.windows(length).step_by(shift),
            sample_rate: self.sample_rate,
        }
    }
}

use std::iter::StepBy;
use std::slice::Windows;
pub struct Frames<'a> {
    it: StepBy<Windows<'a, Complex64>>,
    sample_rate: usize,
}

pub struct FrameSlice<'a> {
    frame: &'a [Complex64],
    sample_rate: usize,
}

pub struct WindowedFrame<'a> {
    frame: &'a [Complex64],
    sample_rate: usize,
    window: &'a Window,
}

pub struct Windoweds<'a> {
    frames: Frames<'a>,
    window: Window,
}

impl<'a> Frames<'a> {
    pub fn windoweds(self, window: Window) -> Windoweds<'a> {
        Windoweds {
            frames: self,
            window,
        }
    }
}

impl<'a> Iterator for Frames<'a> {
    type Item = FrameSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|frame| FrameSlice {
            frame,
            sample_rate: self.sample_rate,
        })
    }
}

// impl<'a> Iterator for Windoweds<'a> {
//     type Item = WindowedFrame<'a>;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         self.frames.next().map(|frameslice| WindowedFrame {
//             frame: frameslice.frame,
//             window: &self.window,
//             sample_rate: frameslice.sample_rate,
//         })
//     }
// }

impl<'a> FrameSlice<'a> {
    pub fn as_slice(&self) -> &'a [Complex64] {
        self.frame
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
    fn test_shift1() {
        let v = Signal::new(vec![
            Complex::new(1., 2.),
            Complex::new(2., 3.),
            Complex::new(3., 4.),
            Complex::new(4., 1.),
        ]);
        let v1 = v.shift(1);
        assert_eq!(
            v1,
            Signal::new(vec![
                Complex::new(0., 0.),
                Complex::new(1., 2.),
                Complex::new(2., 3.),
                Complex::new(3., 4.)
            ])
        );
    }

    #[test]
    fn test_shift2() {
        let v = Signal::from_reals(vec![1., 2., 3., 4.], 4);
        let v1 = v.shift(-1);
        assert_eq!(
            v1,
            Signal::new(vec![
                Complex::new(2., 0.),
                Complex::new(3., 0.),
                Complex::new(4., 0.),
                Complex::new(0., 0.)
            ])
        );
    }

    #[test]
    fn test_integration() {
        let v = Signal::new(vec![
            Complex::new(1., 2.),
            Complex::new(2., -4.),
            Complex::new(3., -6.),
            Complex::new(4., 8.),
        ]);
        let v2 = v.integrate();
        assert_eq!(v2.len(), 4);
        assert_eq!(
            v2,
            Signal::new(vec![
                Complex::new(1., 2.),
                Complex::new(3., -2.),
                Complex::new(6., -8.),
                Complex::new(10., 0.)
            ])
        );
    }

    #[test]
    fn test_differentiation() {
        let v = Signal::new(vec![
            Complex::new(1., 2.),
            Complex::new(2., -4.),
            Complex::new(3., -6.),
            Complex::new(4., 8.),
        ]);
        let v2 = v.differentiate();
        assert_eq!(v2.len(), 4);
        assert_eq!(
            v2,
            Signal::new(vec![
                Complex::new(1., 2.),
                Complex::new(1., -6.),
                Complex::new(1., -2.),
                Complex::new(1., 14.)
            ])
        );
    }

    #[test]
    fn test_energy() {
        let v = Signal::new(vec![
            Complex::new(1., 1.),
            Complex::new(2., -1.),
            Complex::new(1., -1.),
            Complex::new(1., -2.),
        ]);
        assert_eq!(v.energy(), 14.0);
    }

    #[test]
    fn test_power() {
        let v = Signal::new(vec![
            Complex::new(1., 1.),
            Complex::new(2., -1.),
            Complex::new(1., -1.),
            Complex::new(1., -2.),
        ]);
        assert_eq!(v.power(), 14. / 4.);
    }

    #[test]
    fn test_reverse() {
        let v = Signal::new(vec![
            Complex::new(3., 13.),
            Complex::new(2., 4.),
            Complex::new(1., 5.),
        ]);
        assert_eq!(
            v.reverse(),
            Signal::new(vec![
                Complex::new(1., 5.),
                Complex::new(2., 4.),
                Complex::new(3., 13.)
            ])
        );
    }

    #[test]
    fn test_convolve() {
        let u = Signal::new(vec![
            Complex::new(1., 0.),
            Complex::new(1., 0.),
            Complex::new(1., 0.),
            Complex::new(1., 0.),
        ]);
        let h = Signal::new(vec![
            Complex::new(3., 0.),
            Complex::new(2., 0.),
            Complex::new(1., 0.),
        ]);
        println!("Convolved {:?}", u.convolve(&h));
        assert_eq!(
            u.convolve(&h),
            Signal::new(vec![
                Complex::new(3., 0.),
                Complex::new(5., 0.),
                Complex::new(6., 0.),
                Complex::new(6., 0.)
            ])
        );
    }

}
