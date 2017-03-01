/// Helper functions for signal processing
///

use std::ops::Mul;
use std::cmp;
use ndarray::{Array, Ix1};
use num_complex::{Complex, Complex32};


/// One dimensional signal
pub type Signal = Array<Complex32, Ix1>;

/// Create new signal from vector data.
/// This vector will be owned by Signal.
pub fn signal(v: Vec<Complex32>) -> Signal {
    Array::from_vec(v)
}

/// Create new signal from vector data.
/// This vector will be owned by Signal.
pub fn real_signal(v: Vec<f32>) -> Signal {
    let v2: Vec<Complex32> = v.iter().map(|x| Complex::new(*x, 0.)).collect();
    signal(v2)
}

pub trait SignalImpl {
    /// Embed finite time series into infinite one. Pad with zeros
    fn embedded_get(&self, i: isize) -> Complex32;

    /// Shift signal by given integer
    /// y[n] = x[n-k]
    fn shift(&self, k: isize) -> Signal;

    /// Scale signal by given value
    /// y[n] = a*x[n]
    fn scale(&self, k: f32) -> Signal;

}

impl SignalImpl for Signal {
    fn embedded_get(&self, i: isize) -> Complex32 {
        let s = self.len() as isize;
        if i < 0 || i >= s {
            Complex::new(0., 0.)
        } else {
            self.get(i as usize).map(|x| *x).unwrap_or(Complex::new(0., 0.))
        }
    }

    fn shift(&self, k: isize) -> Signal {
        let mut v: Vec<Complex32> = Vec::with_capacity(self.len());
        let size: isize = self.len() as isize;
        for n in 0..size {
            v.push(self.embedded_get(n-k));
        }
        signal(v)
    }

    fn scale(&self, a: f32) -> Signal {
        self.mul(a)
    }

}

/// Add 2 signals
/// z[n] = x[n] + y[n]
pub fn add(v1: &Signal, v2: &Signal) -> Signal {
    let size = cmp::max(v1.len(), v2.len());
    let mut x: Vec<Complex32> = Vec::with_capacity(size);
    for n in 0..size {
        x.push(v1.embedded_get(n as isize) + v2.embedded_get(n as isize));
    }
    signal(x)
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use num_complex::{Complex};
    use super::*;

    #[test]
    fn test_init() {
        let v = real_signal(vec![1., 2., 3., 4.]);
        assert!(v.ndim() == 1);
        assert!(v.len() == 4);
        assert!(v == signal(vec![Complex::new(1., 0.),
                                 Complex::new(2., 0.),
                                 Complex::new(3., 0.),
                                 Complex::new(4., 0.)]));
    }

    #[test]
    fn test_shift1() {
        let v = signal(vec![Complex::new(1., 2.),
                            Complex::new(2., 3.),
                            Complex::new(3., 4.),
                            Complex::new(4., 1.)]);
        let v1 = v.shift(1);
        assert!(v1.ndim() == 1);
        assert!(v1 == signal(vec![Complex::new(0., 0.),
                                  Complex::new(1., 2.),
                                  Complex::new(2., 3.),
                                  Complex::new(3., 4.)]));
    }

    #[test]
    fn test_shift2() {
        let v = real_signal(vec![1., 2., 3., 4.]);
        let v1 = v.shift(-1);
        assert!(v1.ndim() == 1);
        assert!(v1 == signal(vec![Complex::new(2., 0.),
                                  Complex::new(3., 0.),
                                  Complex::new(4., 0.),
                                  Complex::new(0., 0.)]));
    }

    #[test]
    fn test_scale() {
        let v = real_signal(vec![1., 2., 3., 4.]);
        let v1 = v.scale(-2.0);
        assert!(v1.ndim() == 1);
        assert!(v1 == signal(vec![Complex::new(-2., 0.),
                                  Complex::new(-4., 0.),
                                  Complex::new(-6., 0.),
                                  Complex::new(-8., 0.)]));
    }

    #[test]
    fn test_add() {
        let x = signal(vec![Complex::new(1., 2.),
                            Complex::new(2., 4.),
                            Complex::new(3., 6.),
                            Complex::new(4., 8.)]);
        let y = real_signal(vec![2., 3., 4.]);
        let z = add(&x, &y);
        assert!(z.ndim() == 1);
        assert!(z == signal(vec![Complex::new(3., 2.),
                                 Complex::new(5., 4.),
                                 Complex::new(7., 6.),
                                 Complex::new(4., 8.)]));
    }
}