/// Spatial domain processing (e.g. Time)

use std::ops::Mul;
use ndarray::{Array, Ix1};


/// One dimensional signal
pub type Vector = Array<f32, Ix1>;

/// Create new signal from vector data.
/// This vector will be owned by Signal.
pub fn signal(v: Vec<f32>) -> Vector {
    Array::from_vec(v)
}

pub trait Signal {
    /// Extended finite time series into infinite one. Pad with zeros
    fn extended_get(&self, i: isize) -> f32;

    /// Shift signal by given integer
    /// y[n] = x[n-k]
    fn shift(&self, k: isize) -> Vector;

    /// Scale signal by given value
    /// y[n] = x[n-k]
    fn scale(&self, k: f32) -> Vector;
}

impl Signal for Vector {
    fn extended_get(&self, i: isize) -> f32 {
        let s = self.len() as isize;
        if i < 0 || i >= s {0.0} else {self.get(i as usize).map(|x| *x).unwrap_or(0.0)}
    }

    fn shift(&self, k: isize) -> Vector {
        let mut v: Vec<f32> = Vec::with_capacity(self.len());
        let size: isize = self.len() as isize;
        for n in 0..size {
            v.push(self.extended_get(n-k));
        }
        signal(v)
    }

    fn scale(&self, a: f32) -> Vector {
        self.mul(a)
    }
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
}