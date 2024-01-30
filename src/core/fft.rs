//! Helper functions for FFT.
use std::sync::Arc;
use crate::{node::ProcessNode, window};
use rustfft::{Fft, FftPlanner};
use crate::num_complex::Complex32;


pub struct ForwardFFT {
    fft: Arc<dyn Fft<f32>>,
    window: window::Window
}

pub enum WindowType {
    Blackman,
    Hamming,
    Hann,
    Rectangular,
    Welch,
}

impl ForwardFFT {
    /// Define new transformation
    /// ## Params:
    ///   * sample_size - Size of the vector which will be converter. Should be power of 2 (or 3)
    pub fn new(sample_size: usize, window_type: WindowType) -> ForwardFFT {
        let window = match window_type {
            WindowType::Blackman => window::blackman(sample_size),
            WindowType::Hamming => window::hamming(sample_size),
            WindowType::Hann => window::hann(sample_size),
            WindowType::Welch => window::welch(sample_size),
            _ => window::rectangular(sample_size),
        };
        let mut fft = FftPlanner::new();
        ForwardFFT { fft: fft.plan_fft_forward(sample_size), window }
    }
}

impl ProcessNode<Complex32, Complex32> for ForwardFFT {

    fn process_buffer(&mut self, input_buffer: &[Complex32], output_buffer: &mut [Complex32]) {
        let n = usize::min(usize::min(input_buffer.len(), output_buffer.len()), self.window.len());
        for i in 0..n {
            output_buffer[i] = input_buffer[i].scale(self.window.as_slice()[i]); 
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

impl ProcessNode<Complex32, Complex32> for InverseFFT {

    fn process_buffer(&mut self, input_buffer: &[Complex32], output_buffer: &mut [Complex32]) {
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
    use crate::node::ProcessNode;

    #[test]
    fn test_fft() {
        let input_buffer = vec![
            Complex32::new(1., 0.), 
            Complex32::new(0., 0.), 
            Complex32::new(0., 0.), 
            Complex32::new(0., 0.)];
        let mut output_buffer = vec![Complex32::new(0., 0.); 4];
        
        let mut ft = ForwardFFT::new(4, WindowType::Rectangular);
        let _ = ft.process_buffer(&input_buffer, &mut output_buffer);
        let expected = vec![Complex32::new(1., 0.); 4];
        assert_eq!(&output_buffer, &expected);
    }
}
