/// Hilbert space is a vector space where Digital Signals are processed.
/// Here vector space is defined over set of Complex numbers.

use std::ops::Mul;
use std::cmp;
use std::f64::consts::PI;
use ndarray::{Array, Ix1};
use num_complex::{Complex, Complex64};


/// One dimensional signal
pub type Vector = Array<Complex64, Ix1>;

/// Create vector from complex numbers
pub fn vector(v: Vec<Complex64>) -> Vector {
    Array::from_vec(v)
}

/// Create new vector from real numbers.
pub fn from_real(v: Vec<f64>) -> Vector {
    let v2: Vec<Complex64> = v.iter().map(|x| Complex::new(*x, 0.)).collect();
    vector(v2)
}

pub trait VectorImpl {
    /// Embed finite time series into infinite one. Pad with zeros
    fn safe_get(&self, i: isize) -> Complex64;

    /// Shift signal by given integer
    /// y[n] = x[n-k]
    fn shift(&self, k: isize) -> Vector;

    /// Scale signal by given value
    /// y[n] = a*x[n]
    fn scale(&self, k: f64) -> Vector;

}

impl VectorImpl for Vector {
    fn safe_get(&self, i: isize) -> Complex64 {
        let s = self.len() as isize;
        if i < 0 || i >= s {
            Complex::new(0., 0.)
        } else {
            self.get(i as usize).map(|x| *x).unwrap_or(Complex::new(0., 0.))
        }
    }

    fn shift(&self, k: isize) -> Vector {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.len());
        let size: isize = self.len() as isize;
        for n in 0..size {
            v.push(self.safe_get(n-k));
        }
        vector(v)
    }

    fn scale(&self, a: f64) -> Vector {
        self.mul(a)
    }

}

/// Add 2 vectors
/// z[n] = x[n] + y[n]
pub fn add(v1: &Vector, v2: &Vector) -> Vector {
    let size = cmp::max(v1.len(), v2.len());
    let mut x: Vec<Complex64> = Vec::with_capacity(size);
    for n in 0..size {
        x.push(v1.safe_get(n as isize) + v2.safe_get(n as isize));
    }
    vector(x)
}


/// Multiply 2 vectors element wise
/// z[n] = x[n] * y[n]
pub fn multiply(v1: &Vector, v2: &Vector) -> Vector {
    let size = cmp::min(v1.len(), v2.len());
    let mut x: Vec<Complex64> = Vec::with_capacity(size);
    for n in 0..size {
        x.push(v1.safe_get(n as isize) * v2.safe_get(n as isize));
    }
    vector(x)
}


/// Base function for Discrete Fourier Transformation
pub fn fourier_base(size: usize, k: usize) -> Vector {
    assert!(size > 0);
    let w = 2.0*PI/(size as f64)*(k as f64);
    let mut vs: Vec<Complex64> = Vec::with_capacity(size);

    for n in 0..size {
        vs.push(Complex::new(0., -w*(n as f64)).exp());
    }
    vector(vs)
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
        let v = from_real(vec![1., 2., 3., 4.]);
        assert!(v.ndim() == 1);
        assert!(v.len() == 4);
        assert!(v == vector(vec![Complex::new(1., 0.),
                                 Complex::new(2., 0.),
                                 Complex::new(3., 0.),
                                 Complex::new(4., 0.)]));
    }

    #[test]
    fn test_shift1() {
        let v = vector(vec![Complex::new(1., 2.),
                            Complex::new(2., 3.),
                            Complex::new(3., 4.),
                            Complex::new(4., 1.)]);
        let v1 = v.shift(1);
        assert!(v1.ndim() == 1);
        assert!(v1 == vector(vec![Complex::new(0., 0.),
                                  Complex::new(1., 2.),
                                  Complex::new(2., 3.),
                                  Complex::new(3., 4.)]));
    }

    #[test]
    fn test_shift2() {
        let v = from_real(vec![1., 2., 3., 4.]);
        let v1 = v.shift(-1);
        assert!(v1.ndim() == 1);
        assert!(v1 == vector(vec![Complex::new(2., 0.),
                                  Complex::new(3., 0.),
                                  Complex::new(4., 0.),
                                  Complex::new(0., 0.)]));
    }

    #[test]
    fn test_scale() {
        let v = from_real(vec![1., 2., 3., 4.]);
        let v1 = v.scale(-2.0);
        assert!(v1.ndim() == 1);
        assert!(v1 == vector(vec![Complex::new(-2., 0.),
                                  Complex::new(-4., 0.),
                                  Complex::new(-6., 0.),
                                  Complex::new(-8., 0.)]));
    }

    #[test]
    fn test_add() {
        let x = vector(vec![Complex::new(1., 2.),
                            Complex::new(2., 4.),
                            Complex::new(3., 6.),
                            Complex::new(4., 8.)]);
        let y = from_real(vec![2., 3., 4.]);
        let z = add(&x, &y);
        assert!(z.ndim() == 1);
        assert!(z == vector(vec![Complex::new(3., 2.),
                                 Complex::new(5., 4.),
                                 Complex::new(7., 6.),
                                 Complex::new(4., 8.)]));
    }

    #[test]
    fn test_multiply() {
        let x = vector(vec![Complex::new(1., 2.),
                            Complex::new(2., 4.),
                            Complex::new(3., 6.),
                            Complex::new(4., 8.)]);
        let y = vector(vec![Complex::new(2., 4.),
                            Complex::new(3., 6.),
                            Complex::new(4., 1.)]);
        let z = multiply(&x, &y);
        assert!(z.ndim() == 1);
        assert!(z == vector(vec![Complex::new(-6., 8.),
                                 Complex::new(-18., 24.),
                                 Complex::new(6., 27.)]));
    }

    #[test]
    fn test_dft_base_0() {
        let xs = fourier_base(4, 0);
        assert!(xs == vector(vec![Complex::new(1., 0.),
                                  Complex::new(1., 0.),
                                  Complex::new(1., 0.),
                                  Complex::new(1., 0.)]));
    }

    #[test]
    fn test_dot() {
        let xs = vector(vec![Complex::new(1., 1.),
                            Complex::new(3., -1.)]);
        assert!(inner_product(&xs, &xs) == 12.);
    }

}