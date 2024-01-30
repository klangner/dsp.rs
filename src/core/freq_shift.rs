//! Node for shifting frequency of a giveen signal
//! 

use anyhow::Result;
use num_complex::Complex32;
use crate::node::ProcessNode;

use super::generator::Sine;


/// Multiply buffer sample by constant value
/// 
/// Example
/// 
/// ```
/// use dsp::num_complex::Complex32;
/// use dsp::node::ProcessNode;
/// use dsp::core::generator::Sine;
/// use dsp::core::freq_shift::FrequencyShift;
/// 
/// let mut node = FrequencyShift::new(30., 1024);
/// let input_buffer = vec![Complex32::default(); 1024];
/// let mut output_buffer = vec![Complex32::default(); 1024];
/// let source = Sine::new(10., 1024);
/// node.process_buffer(&input_buffer, &mut output_buffer);
/// ```
pub struct FrequencyShift {
    offset_signal: Sine,
}

impl FrequencyShift {
    pub fn new(freq_offset: f32, sample_rate: usize) -> Self {
        let offset_signal = Sine::new(freq_offset as f32, sample_rate);
        Self {offset_signal}
    }
}

impl ProcessNode<Complex32, Complex32> for FrequencyShift {
    fn process_buffer(&mut self, input_buffer: &[Complex32], output_buffer: &mut [Complex32]) -> Result<()> {
        let n = usize::min(input_buffer.len(), output_buffer.len());
        for i in 0..n {
            output_buffer[i] = self.offset_signal.next().unwrap() * input_buffer[i]; 
        }
        
        Ok(())
    }
}
