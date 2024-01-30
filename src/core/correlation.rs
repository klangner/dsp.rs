//! Calculate (Auto)Correlation
//! 
use crate::node::ProcessNode;


pub struct AutoCorrelation {
    window_size: usize,
}

impl AutoCorrelation {
    pub fn new(window_size: usize) -> AutoCorrelation {
        AutoCorrelation {window_size}
    }
}

impl ProcessNode<f32, f32> for AutoCorrelation {

    /// Calculate correlation between 2 buffers
    /// 
    /// Example
    /// 
    /// ```
    /// use assert_approx_eq::assert_approx_eq;
    /// use dsp::node::{SourceNode, ProcessNode};
    /// use dsp::core::generator::Sine;
    /// use dsp::core::correlation::AutoCorrelation;
    /// 
    /// let mut signal = Sine::new(2.0, 8);
    /// let mut corr = AutoCorrelation::new(8);
    /// let mut buffer = vec![0.0;16];
    /// let mut corr_buffer = vec![0.; 8];
    /// let _ = signal.write_buffer(&mut buffer);
    /// let _ = corr.process_buffer(&buffer, &mut corr_buffer);
    /// 
    /// assert_approx_eq!(corr_buffer[0], 1.0, 1e-5f32);
    /// assert_approx_eq!(corr_buffer[1], 0., 1e-5f32);
    /// assert_approx_eq!(corr_buffer[2], -1.0, 1e-5f32);
    /// assert_approx_eq!(corr_buffer[3], 0., 1e-5f32);
    /// assert_approx_eq!(corr_buffer[4], 1.0, 1e-5f32);
    /// ```
    fn process_buffer(&mut self, input_buffer: &[f32], output_buffer: &mut [f32]) {
        let mu: f32 = input_buffer.iter().sum::<f32>() / input_buffer.len() as f32;
        let max_offset = usize::min(input_buffer.len()-self.window_size, output_buffer.len());
        // auto covariance
        for i in 0..max_offset {
            output_buffer[i] = (0..self.window_size)
                .map(|j| (input_buffer[j] - mu) * (input_buffer[j+i] - mu))
                .sum::<f32>() / self.window_size as f32;
        }
        // normalize
        let s0 = output_buffer[0];
        for i in 0..output_buffer.len() {
            output_buffer[i] = output_buffer[i] / s0;
        }
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
}
