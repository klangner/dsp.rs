//! Helper functions for FFT.
use std::sync::Arc;
use rustfft::{Fft, FftPlanner};
use crate::num_complex::Complex32;
use crate::block::{ProcessBlock};


pub struct ForwardFFT {
    fft: Arc<dyn Fft<f32>>,
}

impl ForwardFFT {
    /// Define new transformation
    /// ## Params:
    ///   * sample_size - Size of the vector which will be converter. Should be power of 2 (or 3)
    pub fn new(sample_size: usize) -> ForwardFFT {
        let mut fft = FftPlanner::new();
        ForwardFFT {
            fft: fft.plan_fft_forward(sample_size),
        }
    }
}

impl ProcessBlock<Complex32, Complex32> for ForwardFFT {

    fn process_buffer(&self, input_buffer: &[Complex32], output_buffer: &mut [Complex32]){
        let n = usize::min(input_buffer.len(), output_buffer.len());
        for i in 0..n {
            output_buffer[i] = input_buffer[i]; 
        }
        self.fft.process(output_buffer);
    }
}


pub struct InverseFFT {
    fft: Arc<dyn Fft<f32>>,
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
}

impl ProcessBlock<Complex32, Complex32> for InverseFFT {

    fn process_buffer(&self, input_buffer: &[Complex32], output_buffer: &mut [Complex32]){
        let n = usize::min(input_buffer.len(), output_buffer.len());
        for i in 0..n {
            output_buffer[i] = input_buffer[i]; 
        }
        self.fft.process(output_buffer);
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::ProcessBlock;

    #[test]
    fn test_fft() {
        let input_buffer = vec![
            Complex32::new(1., 0.), 
            Complex32::new(0., 0.), 
            Complex32::new(0., 0.), 
            Complex32::new(0., 0.)];
        let mut output_buffer = vec![Complex32::new(0., 0.); 4];
        
        let ft = ForwardFFT::new(4);
        ft.process_buffer(&input_buffer, &mut output_buffer);
        let expected = vec![Complex32::new(1., 0.); 4];
        assert_eq!(&output_buffer, &expected);
    }
}
