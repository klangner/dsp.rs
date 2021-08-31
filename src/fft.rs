//! Helper functions for FFT.
use std::sync::Arc;
use rustfft::{Fft, FftPlanner};
use crate::num_complex::Complex32;
use crate::signal::Signal;
use crate::spectrum::Spectrum;
use crate::{ComplexBuffer, RealBuffer};


pub struct ForwardFFT {
    fft: Arc<dyn Fft<f32>>,
}

pub struct InverseFFT {
    fft: Arc<dyn Fft<f32>>,
}

impl ForwardFFT {
    /// Define new transformation
    /// ## Params:
    ///   * sample_rate - Samples per second (1/sample_frequency)
    ///   * sample_size - Size of the vector which will be converter. Should be power of 2 (or 3)
    pub fn new(sample_size: usize) -> ForwardFFT {
        let mut fft = FftPlanner::new();
        ForwardFFT {
            fft: fft.plan_fft_forward(sample_size),
        }
    }

    /// Forward DFT (implemented as FFT)
    pub fn process(&mut self, signal: &Signal) -> Spectrum {
        let mut output: ComplexBuffer = signal.data.iter().map(|i| Complex32::new(*i, 0.0)).collect();
        self.fft.process(&mut output);
        Spectrum::new(output, signal.sample_rate)
    }

    /// Forward DFT (implemented as FFT)
    pub fn process_real(&mut self, input: &[f32]) -> RealBuffer {
        let mut output: ComplexBuffer = input.iter().map(|i| Complex32::new(*i, 0.0)).collect();
        self.fft.process(&mut output);
        output.iter().map(|c| c.norm()).collect()
    }
}

impl InverseFFT {
    /// Define new transformation
    /// ## Params:
    ///   * sample_size - Size of the vector which will be converter. Should be power of 2 (or 3)
    pub fn new(sample_size: usize) -> InverseFFT {
        let mut fft = FftPlanner::new();
        InverseFFT {
            fft: fft.plan_fft_inverse(sample_size),
        }
    }

    /// Forward DFT (implemented as FFT)
    pub fn process(&mut self, spectrum: &Spectrum) ->  Signal {
        let mut output: ComplexBuffer = spectrum.data.iter().map(|_| Complex32::new(0.0, 0.0)).collect();
        self.fft.process(&mut output);
        Signal::new(output.iter().map(|c| c.re).collect(), spectrum.sample_rate)
    }

    /// Forward DFT (implemented as FFT)
    pub fn process_real(&mut self, input: &[f32]) ->  RealBuffer {
        let mut output: ComplexBuffer = input.iter().map(|_| Complex32::new(0.0, 0.0)).collect();
        self.fft.process(&mut output);
        output.iter().map(|c| c.re).collect()
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft() {
        let signal = Signal::new(vec![1., 0., 0., 0.], 4);
        
        let mut ft = ForwardFFT::new(4);
        let spectrum = ft.process(&signal);
        let expected: Vec<f32> = vec![1.0, 1.0, 1.0, 1.0];
        assert_eq!(spectrum.sample_rate, signal.sample_rate);
        assert_eq!(&spectrum.to_real(), &expected);
    }
}
