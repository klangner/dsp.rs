use num_complex::{Complex, Complex64};
/// Helper functions for working with Vec as Vector in Vector (Hilbert) Space.
/// Here vector space is defined over set of Complex numbers.
use std::cmp;
use std::f64;

pub type Vector = Vec<Complex64>;

pub trait VectorImpl {
    /// This function will return 0 if index is out of bounds
    fn at(&self, i: usize) -> Complex64;

    /// Scale vector by given value
    /// y[n] = a*x[n]
    fn scale(&self, a: f64) -> Vector;

    /// Add 2 vectors together
    /// z[n] = x[n] + y[n]
    fn add(&self, v: &Vector) -> Vector;

    /// Multiply 2 vectors element wise
    /// z[n] = x[n] * y[n]
    fn multiply(&self, v: &Vector) -> Vector;

    /// Sum elements of the vector
    fn sum(&self) -> Complex64;

    /// Inner product between 2 vectors
    fn inner_product(&self, v: &Vector) -> Complex64;

    /// Get value with max magnitude
    fn max(&self) -> f64;

    /// Get argument of maximum value
    fn argmax(&self) -> usize;
}

impl VectorImpl for Vector {
    fn at(&self, i: usize) -> Complex64 {
        if i < self.len() {
            self[i]
        } else {
            Complex::new(0., 0.)
        }
    }

    fn scale(&self, a: f64) -> Vector {
        let data = self.to_vec().iter().map(|x| x * a).collect();
        data
    }

    // TODO: this can be faster using what is available between multithreading, GPU or SIMD instructions
    fn add(&self, v: &Vector) -> Vector {
        let size = cmp::max(self.len(), v.len());
        let mut x: Vec<Complex64> = Vec::with_capacity(size);
        for n in 0..size {
            x.push(self.at(n) + v.at(n));
        }
        x
    }

    fn multiply(&self, v: &Vector) -> Vector {
        let size = cmp::min(self.len(), v.len());
        let mut x: Vec<Complex64> = Vec::with_capacity(size);
        for n in 0..size {
            x.push(self.at(n) * v.at(n));
        }
        x
    }

    fn sum(&self) -> Complex64 {
        self.iter().fold(Complex::new(0.0, 0.0), |acc, v| acc + v)
    }

    fn inner_product(&self, v: &Vector) -> Complex64 {
        self.multiply(v).sum()
    }

    fn max(&self) -> f64 {
        self.iter()
            .map(|x| x.norm())
            .fold(f64::MIN, |acc, v| if acc < v { v } else { acc })
    }

    fn argmax(&self) -> usize {
        let range = self.iter().map(|x| x.norm()).enumerate();
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
    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_init() {
        let v: Vector = vec![1., 2., 3., 4.]
            .iter()
            .map(|x| Complex::new(*x, 0.))
            .collect();
        assert_eq!(
            v,
            vec![
                Complex::new(1., 0.),
                Complex::new(2., 0.),
                Complex::new(3., 0.),
                Complex::new(4., 0.)
            ]
        );
    }

    #[test]
    fn test_scale() {
        let v: Vector = vec![1., 2., 3., 4.]
            .iter()
            .map(|x| Complex::new(*x, 0.))
            .collect();
        let v1 = v.scale(-2.0);
        assert_eq!(
            v1,
            vec![
                Complex::new(-2., 0.),
                Complex::new(-4., 0.),
                Complex::new(-6., 0.),
                Complex::new(-8., 0.)
            ]
        );
    }

    #[test]
    fn test_add() {
        let x = vec![
            Complex::new(1., 2.),
            Complex::new(2., 4.),
            Complex::new(3., 6.),
            Complex::new(4., 8.),
        ];
        let y: Vector = vec![2., 3., 4.]
            .iter()
            .map(|x| Complex::new(*x, 0.))
            .collect();
        let z = x.add(&y);
        assert_eq!(
            z,
            vec![
                Complex::new(3., 2.),
                Complex::new(5., 4.),
                Complex::new(7., 6.),
                Complex::new(4., 8.)
            ]
        );
    }

    #[test]
    fn test_multiply() {
        let x = vec![
            Complex::new(1., 2.),
            Complex::new(2., 4.),
            Complex::new(3., 6.),
            Complex::new(4., 8.),
        ];
        let y = vec![
            Complex::new(2., 4.),
            Complex::new(3., 6.),
            Complex::new(4., 1.),
        ];
        let z = x.multiply(&y);
        assert_eq!(
            z,
            vec![
                Complex::new(-6., 8.),
                Complex::new(-18., 24.),
                Complex::new(6., 27.)
            ]
        );
    }

    #[test]
    fn test_sum() {
        let x = vec![
            Complex::new(1., 2.),
            Complex::new(3., 4.),
            Complex::new(-3., -2.),
            Complex::new(4., 2.),
        ];
        assert_eq!(x.sum(), Complex::new(5.0, 6.0));
    }

    #[test]
    fn test_inner_product() {
        let x = vec![Complex::new(1., 2.), Complex::new(2., 4.)];
        let y = vec![Complex::new(2., 4.), Complex::new(3., -6.)];
        assert_eq!(x.inner_product(&y), Complex::new(24., 8.));
    }

    #[test]
    fn test_max() {
        let x = vec![
            Complex::new(1., 2.),
            Complex::new(3., 4.),
            Complex::new(3., 2.),
            Complex::new(4., 2.),
        ];
        assert_eq!(x.max(), 5.0);
    }

    #[test]
    fn test_argmax() {
        let x = vec![
            Complex::new(1., 2.),
            Complex::new(3., 4.),
            Complex::new(3., 2.),
            Complex::new(4., 2.),
        ];
        assert_eq!(x.argmax(), 1);
    }
}
