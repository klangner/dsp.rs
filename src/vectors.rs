/// Vector Space (Hilbert space) discrete signal representation.
/// Here vector space is defined over set of Complex numbers.

use std::cmp;
use std::f64::consts::PI;
use num_complex::{Complex, Complex64};


/// One dimensional signal
#[derive(Debug, PartialEq)]
pub struct Vector {
    data: Vec<Complex64>
}

impl Vector {
    /// Create new vector from real numbers.
    pub fn new(data: Vec<Complex64>) -> Vector {
        Vector { data: data }
    }

    /// Create new vector from real numbers.
    pub fn from_reals(v: Vec<f64>) -> Vector {
        let data: Vec<Complex64> = v.iter().map(|x| Complex::new(*x, 0.)).collect();
        Vector::new(data)
    }

    /// Convert vector into Vec representation
    pub fn to_vec(&self) -> &Vec<Complex64> {
        &self.data
    }

    /// Vector length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Embed finite time series into infinite one. Pad with zeros
    pub fn safe_get(&self, i: isize) -> Complex64 {
        let s = self.data.len() as isize;
        if i < 0 || i >= s {
            Complex::new(0., 0.)
        } else {
            self.data[i as usize]
        }
    }

    /// Shift signal by given integer
    /// y[n] = x[n-k]
    pub fn shift(&self, k: isize) -> Vector {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.len());
        let size: isize = self.len() as isize;
        for n in 0..size {
            v.push(self.safe_get(n-k));
        }
        Vector::new(v)
    }

    /// Scale vector by given value
    /// y[n] = a*x[n]
    pub fn scale(&self, a: f64) -> Vector {
        let data = self.data.iter().map(|x| x*a).collect();
        Vector::new(data)
    }

    /// Integrate signal
    /// y[n] = Sum x[k] For all k <= n
    pub fn integrate(&self) -> Vector {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.len());
        let mut acc = Complex::new(0., 0.);
        for n in 0..self.len() {
            acc = acc + self.safe_get(n as isize);
            v.push(acc);
        }
        Vector::new(v)
    }

    /// Differentiate the signal
    /// y[n] = x[n] - x[n-1]
    pub fn differentiate(&self) -> Vector {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.len());
        let mut last = Complex::new(0., 0.);
        for n in 0..self.len() {
            v.push(self.safe_get(n as isize) - last);
            last = self.safe_get(n as isize);
        }
        Vector::new(v)
    }

    /// Calculate energy
    /// E = Sum x[n]^2 For all n
    pub fn energy(&self) -> f64 {
        self.data.iter().fold(0., |acc, &x| acc + (x*x.conj()).re)
    }

    /// Calculate power
    /// P = 1/N Sum x[n]^2 For all n
    pub fn power(&self) -> f64 {
        self.energy() / (self.len() as f64)
    }

    /// Add vector
    /// z[n] = x[n] + y[n]
    pub fn add(&self, v: &Vector) -> Vector {
        let size = cmp::max(self.data.len(), v.len());
        let mut x: Vec<Complex64> = Vec::with_capacity(size);
        for n in 0..size {
            x.push(self.safe_get(n as isize) + v.safe_get(n as isize));
        }
        Vector::new(x)
    }


    /// Multiply 2 vectors element wise
    /// z[n] = x[n] * y[n]
    pub fn multiply(&self, v: &Vector) -> Vector {
        let size = cmp::min(self.data.len(), v.len());
        let mut x: Vec<Complex64> = Vec::with_capacity(size);
        for n in 0..size {
            x.push(self.safe_get(n as isize) * v.safe_get(n as isize));
        }
        Vector::new(x)
    }
}

/// Base function for Discrete Fourier Transformation
pub fn fourier_base(size: usize, k: usize) -> Vector {
    assert!(size > 0);
    let w = 2.0*PI/(size as f64)*(k as f64);
    let mut vs: Vec<Complex64> = Vec::with_capacity(size);

    for n in 0..size {
        vs.push(Complex::new(0., -w*(n as f64)).exp());
    }
    Vector::new(vs)
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
        let v = Vector::from_reals(vec![1., 2., 3., 4.]);
        assert!(v == Vector::new(vec![Complex::new(1., 0.),
                                      Complex::new(2., 0.),
                                      Complex::new(3., 0.),
                                      Complex::new(4., 0.)]));
    }

    #[test]
    fn test_shift1() {
        let v = Vector::new(vec![Complex::new(1., 2.),
                                 Complex::new(2., 3.),
                                 Complex::new(3., 4.),
                                 Complex::new(4., 1.)]);
        let v1 = v.shift(1);
        assert!(v1 == Vector::new(vec![Complex::new(0., 0.),
                                       Complex::new(1., 2.),
                                       Complex::new(2., 3.),
                                       Complex::new(3., 4.)]));
    }

    #[test]
    fn test_shift2() {
        let v = Vector::from_reals(vec![1., 2., 3., 4.]);
        let v1 = v.shift(-1);
        assert!(v1 == Vector::new(vec![Complex::new(2., 0.),
                                       Complex::new(3., 0.),
                                       Complex::new(4., 0.),
                                       Complex::new(0., 0.)]));
    }

    #[test]
    fn test_scale() {
        let v = Vector::from_reals(vec![1., 2., 3., 4.]);
        let v1 = v.scale(-2.0);
        assert!(v1 == Vector::new(vec![Complex::new(-2., 0.),
                                       Complex::new(-4., 0.),
                                       Complex::new(-6., 0.),
                                       Complex::new(-8., 0.)]));
    }

    #[test]
    fn test_add() {
        let x = Vector::new(vec![Complex::new(1., 2.),
                                 Complex::new(2., 4.),
                                 Complex::new(3., 6.),
                                 Complex::new(4., 8.)]);
        let y = Vector::from_reals(vec![2., 3., 4.]);
        let z = x.add(&y);
        assert!(z == Vector::new(vec![Complex::new(3., 2.),
                                      Complex::new(5., 4.),
                                      Complex::new(7., 6.),
                                      Complex::new(4., 8.)]));
    }

    #[test]
    fn test_multiply() {
        let x = Vector::new(vec![Complex::new(1., 2.),
                                 Complex::new(2., 4.),
                                 Complex::new(3., 6.),
                                 Complex::new(4., 8.)]);
        let y = Vector::new(vec![Complex::new(2., 4.),
                                 Complex::new(3., 6.),
                                 Complex::new(4., 1.)]);
        let z = x.multiply(&y);
        assert!(z == Vector::new(vec![Complex::new(-6., 8.),
                                      Complex::new(-18., 24.),
                                      Complex::new(6., 27.)]));
    }

    #[test]
    fn test_dft_base_0() {
        let xs = fourier_base(4, 0);
        assert!(xs == Vector::new(vec![Complex::new(1., 0.),
                                  Complex::new(1., 0.),
                                  Complex::new(1., 0.),
                                  Complex::new(1., 0.)]));
    }

    #[test]
    fn test_integration() {
        let v = Vector::new(vec![Complex::new(1., 2.),
                            Complex::new(2., -4.),
                            Complex::new(3., -6.),
                            Complex::new(4., 8.)]);
        let v2 = v.integrate();
        assert!(v2.len() == 4);
        assert!(v2 == Vector::new(vec![Complex::new(1., 2.),
                                  Complex::new(3., -2.),
                                  Complex::new(6., -8.),
                                  Complex::new(10., 0.)]));
    }

    #[test]
    fn test_differentiation() {
        let v = Vector::new(vec![Complex::new(1., 2.),
                            Complex::new(2., -4.),
                            Complex::new(3., -6.),
                            Complex::new(4., 8.)]);
        let v2 = v.differentiate();
        assert!(v2.len() == 4);
        assert!(v2 == Vector::new(vec![Complex::new(1., 2.),
                                  Complex::new(1., -6.),
                                  Complex::new(1., -2.),
                                  Complex::new(1., 14.)]));
    }

    #[test]
    fn test_energy() {
        let v = Vector::new(vec![Complex::new(1., 1.),
                            Complex::new(2., -1.),
                            Complex::new(1., -1.),
                            Complex::new(1., -2.)]);
        assert!(v.energy() == 14.0);
    }


    #[test]
    fn test_power() {
        let v = Vector::new(vec![Complex::new(1., 1.),
                            Complex::new(2., -1.),
                            Complex::new(1., -1.),
                            Complex::new(1., -2.)]);
        assert!(v.power() == 14./4.);
    }
}