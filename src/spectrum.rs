//! Analyze discrete signal in frequency domain using complex numbers

use num_complex::Complex32;
use crate::vector;


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
    /// 
    /// Example
    /// 
    /// ```
    /// use dsp::fft::ForwardFFT;
    /// use dsp::generator::sine;
    ///
    /// let signal = sine(1024, 20.0, 512);
    /// let mut ft = ForwardFFT::new(1024);
    /// let spectrum = ft.process(&signal);
    /// assert_eq!(spectrum.item_freq(40), 20.0);
    /// ```
    pub fn item_freq(&self, i: usize) -> f32 {
        (i * self.sample_rate) as f32 / self.len() as f32
    }

    /// Return max frequency
    /// 
    /// Example
    /// 
    /// ```
    /// use dsp::fft::ForwardFFT;
    /// use dsp::generator::sine;
    ///
    /// let signal = sine(1024, 28.0, 512);
    /// let mut ft = ForwardFFT::new(1024);
    /// let spectrum = ft.process(&signal);
    /// assert_eq!(spectrum.max_freq(), 28.0);
    /// ```
    pub fn max_freq(&self) -> f32 {
        let idx = vector::argmax(&self.to_real());
        if idx < self.len() / 2 {
            self.item_freq(idx)
        } else {
            self.item_freq(self.len() - idx)
        }
    }

    /// Convert Complex buffer into Real one
    pub fn to_real(&self) -> Vec<f32> {
        self.data.iter().map(|v| v.norm()).collect()
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::fft::ForwardFFT;
    use crate::generator::sine;

    #[test]
    fn test_item_freq() {
        let signal = sine(1024, 20.0, 512);
        let mut ft = ForwardFFT::new(1024);
        let spectrum = ft.process(&signal);
        assert_eq!(spectrum.item_freq(40), 20.0);
    }
}
