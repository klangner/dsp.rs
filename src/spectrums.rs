//! Analyze discrete signal in frequency domain

use num_complex::Complex64;
use vectors::{Vector, VectorImpl};

#[derive(Debug, PartialEq)]
pub struct Spectrum {
    data: Vector,
    sample_rate: usize,
}

impl Spectrum {
    /// Create new signal from vector
    pub fn new(data: Vec<Complex64>, sample_rate: usize) -> Spectrum {
        Spectrum { data, sample_rate }
    }

    /// Spectrum length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Copy data into new vector
    pub fn to_vec(&self) -> Vec<Complex64> {
        self.data.clone()
    }

    /// Calculated frequency of a given component
    pub fn item_freq(&self, i: usize) -> f64 {
        (i * self.sample_rate) as f64 / (self.data.len() as f64)
    }

    /// Convolution.
    /// In frequency domain convolution it equals to the multiplication
    pub fn convolve(&self, other: &Spectrum) -> Spectrum {
        let data = self.data.multiply(&other.data);
        Spectrum::new(data, self.sample_rate)
    }

    /// Return max frequency
    pub fn max_freq(&self) -> f64 {
        let idx = self.data.argmax();
        if idx < self.len() / 2 {
            self.item_freq(idx)
        } else {
            self.item_freq(self.len() - idx)
        }
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
    fn test_freq_0() {
        let s = Spectrum::new(
            vec![
                Complex::new(1., 0.),
                Complex::new(1., 0.),
                Complex::new(1., 0.),
                Complex::new(1., 0.),
            ],
            4,
        );
        assert_eq!(s.item_freq(0), 0.0);
        assert_eq!(s.item_freq(2), 2.0);
    }

}
