//! FM demodulation
//! 

use anyhow::Result;
use num_complex::Complex32;
use crate::node::ProcessNode;


/// Demodulation block using the conjugate delay method
/// See https://en.wikipedia.org/wiki/Detector_(radio)#Quadrature_detector
/// 
/// Example
/// 
/// ```
/// use dsp::num_complex::Complex32;
/// use dsp::node::ProcessNode;
/// use dsp::core::fm::QuadratureDetector;
/// 
/// let mut node = QuadratureDetector::new();
/// let input_buffer = vec![Complex32::default(); 1024];
/// let mut output_buffer = vec![0.; 1024];
/// node.process_buffer(&input_buffer, &mut output_buffer);
/// ```
pub struct QuadratureDetector {
    last_sample: Complex32,
}

impl QuadratureDetector {
    pub fn new() -> Self {
        Self {last_sample: Complex32::default()}
    }
}

impl ProcessNode<Complex32, f32> for QuadratureDetector {
    fn process_buffer(&mut self, input_buffer: &[Complex32], output_buffer: &mut [f32]) -> Result<()> {
        let n = usize::min(input_buffer.len(), output_buffer.len());
        for i in 0..n {
            let v = &input_buffer[i];
            output_buffer[i] = (v * self.last_sample.conj()).arg(); // Obtain phase of x[n] * conj(x[n-1])
            self.last_sample = *v;
        }
        
        Ok(())
    }
}
