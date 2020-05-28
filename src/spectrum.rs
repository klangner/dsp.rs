//! Analyze discrete signal in frequency domain using complex numbers

use num_complex::Complex32;
use crate::vectors;


/// Spectrum of the signal
pub struct Spectrum {
    pub data: Vec<Complex32>,
    pub sample_rate: usize
}

impl Spectrum {

    /// create new spectrum from the given data
    pub fn new(data: Vec<Complex32>, sample_rate: usize) -> Spectrum {
        Spectrum { data, sample_rate }
    }

    /// Length of the spectrum signal
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Calculated frequency of a given component
    pub fn item_freq(&self, i: usize) -> f32 {
        (i * self.sample_rate) as f32 / self.len() as f32
    }

    /// Return max frequency
    pub fn max_freq(&self) -> f32 {
        let idx = vectors::argmax(&self.to_real());
        if idx < self.len() / 2 {
            self.item_freq(idx)
        } else {
            self.item_freq(self.len() - idx)
        }
    }

    /// Convert Complex buffer into Real one
    fn to_real(&self) -> Vec<f32> {
        self.data.iter().map(|v| v.norm()).collect()
    }
}
