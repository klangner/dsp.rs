//! Analyze discrete signal in frequency domain
use std::sync::Arc;
use rustfft::{FFTplanner, FFT};
use crate::num_complex::Complex32;
use crate::{ComplexBuffer, RealBuffer, ProcessingNode};


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


pub struct ForwardFFTNode {
    fft: Arc<FFT<f32>>,
    input_complex: ComplexBuffer,
    output: ComplexBuffer,
}

impl ForwardFFTNode {
    pub fn new(size: usize) -> ForwardFFTNode {
        let mut fft = FFTplanner::new(false);
        let fft = fft.plan_fft(size);
        let input_complex = vec![Complex32::new(0.0, 0.0); size];
        let output = vec![Complex32::new(0.0, 0.0); size];
        ForwardFFTNode {fft, input_complex, output}
    }
}

impl ProcessingNode for ForwardFFTNode {
    type InBuffer = RealBuffer;
    type OutBuffer = ComplexBuffer;
    
    fn process(&mut self, input: &RealBuffer) -> &ComplexBuffer {
        let n = usize::min(input.len(), self.input_complex.len());
        for i in 0..n {
            self.input_complex[i] = Complex32::new(input[i], 0.0);
        }
        self.fft.process(&mut self.input_complex, &mut self.output);
        &self.output
    }
}


pub struct InverseFFTNode {
    fft: Arc<FFT<f32>>,
    input_complex: ComplexBuffer,
    output_complex: ComplexBuffer,
    output: RealBuffer,
}

impl InverseFFTNode {
    pub fn new(size: usize) -> InverseFFTNode {
        let mut fft = FFTplanner::new(true);
        let fft = fft.plan_fft(size);
        let input_complex = vec![Complex32::new(0.0, 0.0); size];
        let output_complex = vec![Complex32::new(0.0, 0.0); size];
        let output = vec![0.0; size];
        InverseFFTNode {fft, input_complex, output_complex, output}
    }
}

impl ProcessingNode for InverseFFTNode {
    type InBuffer = ComplexBuffer;
    type OutBuffer = RealBuffer;
    
    fn process(&mut self, input: &ComplexBuffer) -> &RealBuffer {
        let n = usize::min(input.len(), self.input_complex.len());
        for i in 0..n {
            self.input_complex[i] = input[i];
        }
        self.fft.process(&mut self.input_complex, &mut self.output_complex);;
        for i in 0..n {
            self.output[i] = self.output_complex[i].re;
        }
        &self.output
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::num_complex::Complex;

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
