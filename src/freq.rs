//! Analyze discrete signal in frequency domain

use std::f64::consts::PI;
use num_complex::{Complex, Complex64};
use rustfft::{FFT};
use vectors::{Vector};


pub struct FourierTransform {
    fft: FFT<f64>,
    sample_rate: usize,
    sample_size: usize
}

impl FourierTransform {
    /// Define new transformation
    /// ## Params:
    ///   * sample_rate - Samples per second (1/sample_frequency)
    ///   * sample_size - Size of the vector which will be converter. Should be power of 2 (or 3)
    pub fn forward(sample_rate: usize, sample_size: usize) -> FourierTransform {
        let fft = FFT::new(sample_size, false);
        FourierTransform{ fft: fft, sample_rate: sample_rate, sample_size: sample_size }
    }

    /// Define inverse transformation
    pub fn inverse(sample_rate: usize, sample_size: usize) -> FourierTransform {
        let fft = FFT::new(sample_size, true);
        FourierTransform{ fft: fft, sample_rate: sample_rate, sample_size: sample_size }
    }

    /// Forward DFT (implemented as FFT)
    pub fn process(&mut self, v: &Vector) -> Vector {
        let raw_vec = v.to_vec();
        let mut out = raw_vec.clone();

        self.fft.process(&raw_vec, &mut out);
        Vector::new(out)
    }

    /// Calculated frequncy of a given component
    pub fn item_freq(&self, i: usize) -> f64 {
        (i * self.sample_rate) as f64 / (self.sample_size as f64)
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
    use vectors::{Vector};
    use super::*;

    #[test]
    fn test_fft() {
        let v = Vector::from_reals(vec![1., 0., 0., 0.]);
        let mut ft = FourierTransform::forward(4, 4);
        let s = ft.process(&v);
        assert!(s == Vector::new(vec![Complex::new(1., 0.),
                                      Complex::new(1., 0.),
                                      Complex::new(1., 0.),
                                      Complex::new(1., 0.)]));
    }

    #[test]
    fn test_freq_0() {
        let ft = FourierTransform::forward(4, 4);
        assert!(ft.item_freq(0) == 0.0);
    }

    #[test]
    fn test_freq_10() {
        let ft = FourierTransform::forward(44100, 1024);
        assert!(ft.item_freq(10) == 430.6640625);
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