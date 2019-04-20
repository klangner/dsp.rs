//! Analyze discrete signal in frequency domain
use std::sync::Arc;

use rustfft::{FFTplanner, FFT};
use crate::signals::Signal;
use crate::spectrums::Spectrum;


pub struct ForwardFFT {
    fft: Arc<FFT<f64>>,
}

pub struct InverseFFT {
    fft: Arc<FFT<f64>>,
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
    pub fn process(&mut self, v: &Signal) -> Spectrum {
        let mut raw_vec = v.to_vec();
        let mut out = raw_vec.clone();

        self.fft.process(&mut raw_vec, &mut out);
        Spectrum::new(out, v.sample_rate())
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
    pub fn process(&mut self, v: &Spectrum) -> Signal {
        let mut raw_vec = v.to_vec();
        let mut out = raw_vec.clone();

        self.fft.process(&mut raw_vec, &mut out);
        Signal::new(out)
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;
    use crate::signals::Signal;
    use crate::spectrums::Spectrum;

    #[test]
    fn test_fft() {
        let v = Signal::from_reals(vec![1., 0., 0., 0.], 4);
        let mut ft = ForwardFFT::new(4);
        let s = ft.process(&v);
        assert_eq!(
            s,
            Spectrum::new(
                vec![
                    Complex::new(1., 0.),
                    Complex::new(1., 0.),
                    Complex::new(1., 0.),
                    Complex::new(1., 0.)
                ],
                4
            )
        );
    }

}
