//! Analyze discrete signal in frequency domain
use std::sync::Arc;
use rustfft::{FFTplanner, FFT};
use crate::ComplexBuffer;


pub struct ForwardFFT {
    fft: Arc<FFT<f32>>,
}

pub struct InverseFFT {
    fft: Arc<FFT<f32>>,
}

impl ForwardFFT {
    /// Define new transformation
    /// ## Params:
    ///   * sample_rate - Samples per second (1/sample_frequency)
    ///   * sample_size - Size of the vector which will be converter. Should be power of 2 (or 3)
    pub fn new(sample_size: usize) -> ForwardFFT {
        let mut fft = FFTplanner::new(false);
        ForwardFFT {
            fft: fft.plan_fft(sample_size),
        }
    }

    /// Forward DFT (implemented as FFT)
    pub fn process(&mut self, mut input: &mut ComplexBuffer, mut output: &mut ComplexBuffer) {
        self.fft.process(&mut input, &mut output);
    }
}

impl InverseFFT {
    /// Define new transformation
    /// ## Params:
    ///   * sample_size - Size of the vector which will be converter. Should be power of 2 (or 3)
    pub fn new(sample_size: usize) -> InverseFFT {
        let mut fft = FFTplanner::new(true);
        InverseFFT {
            fft: fft.plan_fft(sample_size),
        }
    }

    /// Forward DFT (implemented as FFT)
    pub fn process(&mut self, mut input: &mut ComplexBuffer, mut output: &mut ComplexBuffer) {
        self.fft.process(&mut input, &mut output);
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
    fn test_fft() {
        let mut input = vec![Complex::new(1., 0.), 
                             Complex::new(0., 0.),
                             Complex::new(0., 0.),
                             Complex::new(0., 0.)];
        let mut output = vec![Complex::new(0., 0.); 4];
        
        let mut ft = ForwardFFT::new(4);
        ft.process(&mut input, &mut output);
        let expected = vec![
                Complex::new(1., 0.),
                Complex::new(1., 0.),
                Complex::new(1., 0.),
                Complex::new(1., 0.)];
        assert_eq!(&output, &expected);
    }
}
