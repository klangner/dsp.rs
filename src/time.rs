//! Process signals in Time Domain

use rand::{random};
use num_complex::{Complex, Complex64};
use vectors::{Vector};


/// DiscreteTimeSignal
pub trait SpatialSignal {

    /// This function will return 0 if index out of bound
    fn get(&self, i: isize) -> Complex64;

    /// Shift signal by given integer
    /// y[n] = x[n-k]
    /// This function will not change the signal length
    fn shift(&self, k: isize) -> Vector;

    /// Integrate signal
    /// y[n] = Sum x[k] For all k <= n
    fn integrate(&self) -> Vector;

    /// Differentiate the signal
    /// y[n] = x[n] - x[n-1]
    fn differentiate(&self) -> Vector;

    /// Calculate energy
    /// E = Sum x[n]^2 For all n
    fn energy(&self) -> f64;

    /// Calculate power
    /// P = 1/N Sum x[n]^2 For all n
    fn power(&self) -> f64;

    /// Add noise to the signal
    fn add_noise(&self, amplitude: f64) -> Vector;
}

impl SpatialSignal for Vector {

    fn get(&self, i: isize) -> Complex64 {
        let s = self.len() as isize;
        if i < 0 || i >= s {
            Complex::new(0., 0.)
        } else {
        self.at(i as usize)
        }
    }

    fn shift(&self, k: isize) -> Vector {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.len());
        let size: isize = self.len() as isize;
        for n in 0..size {
            v.push(self.get(n-k));
        }
        Vector::new(v)
    }

    fn integrate(&self) -> Vector {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.len());
        let mut acc = Complex::new(0., 0.);
        for n in 0..self.len() {
            acc = acc + self.at(n);
            v.push(acc);
        }
        Vector::new(v)
    }

    fn differentiate(&self) -> Vector {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.len());
        let mut last = Complex::new(0., 0.);
        for n in 0..self.len() {
            v.push(self.at(n) - last);
            last = self.at(n);
        }
        Vector::new(v)
    }

    fn energy(&self) -> f64 {
        self.to_vec().iter().fold(0., |acc, &x| acc + (x*x.conj()).re)
    }

    fn power(&self) -> f64 {
        self.energy() / (self.len() as f64)
    }

    fn add_noise(&self, amplitude: f64) -> Vector {
        self.iter().map(|x| x + amplitude).collect()
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