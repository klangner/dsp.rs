//! Analyze discrete signal in frequency domain

use std::f64::consts::PI;
use num_complex::{Complex, Complex64};
use rustfft;
use vectors::{Vector};


/// Convert spatial vector into spectrum
/// This function runs faster if vector size is power of 2
pub fn fft(v: &Vector) -> Vector {
    let raw_vec = v.to_vec();
    let mut fft = rustfft::FFT::new(raw_vec.len(), false);
    let mut spectrum = raw_vec.clone();

    fft.process(&raw_vec, &mut spectrum);
    Vector::new(spectrum)
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
    use vectors::{Vector};
    use super::*;

    #[test]
    fn test_fft() {
        let v = Vector::from_reals(vec![1., 0., 0., 0.]);
        let s = fft(&v);
        assert!(s == Vector::new(vec![Complex::new(1., 0.),
                                      Complex::new(1., 0.),
                                      Complex::new(1., 0.),
                                      Complex::new(1., 0.)]));
    }

    #[test]
    fn test_dft_base_0() {
        let xs = fourier_base(4, 0);
        assert!(xs == Vector::new(vec![Complex::new(1., 0.),
                                       Complex::new(1., 0.),
                                       Complex::new(1., 0.),
                                       Complex::new(1., 0.)]));
    }

}