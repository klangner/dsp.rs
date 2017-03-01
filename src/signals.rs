/// Helper functions for signal processing
///

use std::ops::Mul;
use std::cmp;
use ndarray::{Array, Ix1};


/// One dimensional signal
pub type Signal = Array<f32, Ix1>;

/// Create new signal from vector data.
/// This vector will be owned by Signal.
pub fn signal(v: Vec<f32>) -> Signal {
    Array::from_vec(v)
}

pub trait SignalImpl {
    /// Embed finite time series into infinite one. Pad with zeros
    fn embedded_get(&self, i: isize) -> f32;

    /// Shift signal by given integer
    /// y[n] = x[n-k]
    fn shift(&self, k: isize) -> Signal;

    /// Scale signal by given value
    /// y[n] = a*x[n]
    fn scale(&self, k: f32) -> Signal;

}

impl SignalImpl for Signal {
    fn embedded_get(&self, i: isize) -> f32 {
        let s = self.len() as isize;
        if i < 0 || i >= s {0.0} else {self.get(i as usize).map(|x| *x).unwrap_or(0.0)}
    }

    fn shift(&self, k: isize) -> Signal {
        let mut v: Vec<f32> = Vec::with_capacity(self.len());
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
    let mut x: Vec<f32> = Vec::with_capacity(size);
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
    use super::*;

    #[test]
    fn test_init() {
        let v = signal(vec![1., 2., 3., 4.]);
        assert!(v.ndim() == 1);
        assert!(v.len() == 4);
        assert!(v == signal(vec![1., 2., 3., 4.]));
    }

    #[test]
    fn test_shift1() {
        let v = signal(vec![1., 2., 3., 4.]);
        let v1 = v.shift(1);
        assert!(v1.ndim() == 1);
        assert!(v1 == signal(vec![0., 1., 2., 3.]));
    }

    #[test]
    fn test_shift2() {
        let v = signal(vec![1., 2., 3., 4.]);
        let v1 = v.shift(-1);
        assert!(v1.ndim() == 1);
        assert!(v1 == signal(vec![2., 3., 4.0, 0.]));
    }

    #[test]
    fn test_scale() {
        let v = signal(vec![1., 2., 3., 4.]);
        let v1 = v.scale(-2.0);
        assert!(v1.ndim() == 1);
        assert!(v1 == signal(vec![-2., -4., -6.0, -8.]));
    }

    #[test]
    fn test_add() {
        let x = signal(vec![1., 2., 3., 4.]);
        let y = signal(vec![2., 3., 4.]);
        let z = add(&x, &y);
        assert!(z.ndim() == 1);
        assert!(z == signal(vec![3., 5., 7.0, 4.]));
    }
}