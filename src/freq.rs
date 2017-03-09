/// Analyze discrete signal in frequency domain

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
}