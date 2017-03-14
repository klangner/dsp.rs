/// Vector Space (Hilbert space) discrete signal representation.
/// Here vector space is defined over set of Complex numbers.

use std::cmp;
use std::f64;
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

    /// This function will return 0 if index is out of bounds
    pub fn at(&self, i: usize) -> Complex64 {
        if i < self.data.len() {
            self.data[i]
        } else {
            Complex::new(0., 0.)
        }
    }

    /// Scale vector by given value
    /// y[n] = a*x[n]
    pub fn scale(&self, a: f64) -> Vector {
        let data = self.to_vec().iter().map(|x| x*a).collect();
        Vector::new(data)
    }

    /// Add 2 vectors together
    /// z[n] = x[n] + y[n]
    pub fn add(&self, v: &Vector) -> Vector {
        let size = cmp::max(self.data.len(), v.len());
        let mut x: Vec<Complex64> = Vec::with_capacity(size);
        for n in 0..size {
            x.push(self.at(n) + v.at(n));
        }
        Vector::new(x)
    }

    /// Multiply 2 vectors element wise
    /// z[n] = x[n] * y[n]
    pub fn multiply(&self, v: &Vector) -> Vector {
        let size = cmp::min(self.data.len(), v.len());
        let mut x: Vec<Complex64> = Vec::with_capacity(size);
        for n in 0..size {
            x.push(self.at(n) * v.at(n));
        }
        Vector::new(x)
    }

    /// Get value with max magnitude
    pub fn max(&self) -> f64 {
        self.data.iter()
            .map(|x| x.norm())
            .fold(f64::MIN, |acc, v| if acc < v {v} else {acc})
    }

    /// Get argument of maximum value
    pub fn argmax(&self) -> usize {
        let range = self.data.iter().map(|x| x.norm()).enumerate();
        let mut max_value = f64::MIN;
        let mut arg_max = 0;
        for (i, v) in range {
            if max_value < v {
                max_value = v;
                arg_max = i;
            }
        }
        arg_max
    }
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
    fn test_max() {
        let x = Vector::new(vec![Complex::new(1., 2.),
                                 Complex::new(3., 4.),
                                 Complex::new(3., 2.),
                                 Complex::new(4., 2.)]);
        assert!(x.max() == 5.0);
    }

    #[test]
    fn test_argmax() {
        let x = Vector::new(vec![Complex::new(1., 2.),
                                 Complex::new(3., 4.),
                                 Complex::new(3., 2.),
                                 Complex::new(4., 2.)]);
        assert!(x.argmax() == 1);
    }
}