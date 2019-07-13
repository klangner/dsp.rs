//! # Digital Signal Processing
//!
//! Signals can be processed in Time or Frequency domain
//!

pub mod generators;
pub mod fft;
pub mod spectrums;
pub mod windows;
pub mod signalops;
mod vectors;

use num_complex::Complex32;


/// Time domain data buffer. Uses Real number
pub type RealBuffer = Vec<f32>;

/// Frequency domain data buffer based on complex numbers
pub type ComplexBuffer = Vec<Complex32>;

/// Node which produces signal
pub trait SourceNode {
    type Buffer;
    
    fn next_batch(&mut self, output: &mut Self::Buffer);
}

// signal transformation
pub trait ProcessingNode {
    type InBuffer;
    type OutBuffer;
    
    fn process(&mut self, input: &Self::InBuffer, output: &mut Self::OutBuffer);
}

// Consume signal
pub trait SinkNode {
    type Buffer;
    
    fn consume(&mut self, input: &Self::Buffer);
}

pub struct RealToComplexNode {}

impl RealToComplexNode {
    pub fn new() -> RealToComplexNode {
        RealToComplexNode {}
    }
}

impl ProcessingNode for RealToComplexNode {
    type InBuffer = RealBuffer;
    type OutBuffer = ComplexBuffer;
    
    fn process(&mut self, input: &RealBuffer, output: &mut ComplexBuffer) {
        let n = usize::min(input.len(), output.len());
        for i in 0..n {
            output[i] = Complex32::new(input[i], 0.0);
        }
    }
}

pub struct ComplexToRealNode {}

impl ComplexToRealNode {
    pub fn new() -> ComplexToRealNode {
        ComplexToRealNode {}
    }
}

impl ProcessingNode for ComplexToRealNode {
    type InBuffer = ComplexBuffer;
    type OutBuffer = RealBuffer;
    
    fn process(&mut self, input: &ComplexBuffer, output: &mut RealBuffer) {
        let n = usize::min(input.len(), output.len());
        for i in 0..n {
            output[i] = input[i].re;
        }
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use num_complex::Complex32;
    use super::*;

    #[test]
    fn test_real_to_complex() {
        let real = vec![1.0, 2.0, 3.0, 4.0];
        let mut complex = vec![Complex32::new(0.0, 0.0); 4];
        let expected = vec![Complex32::new(1.0, 0.0), 
                            Complex32::new(2.0, 0.0),
                            Complex32::new(3.0, 0.0),
                            Complex32::new(4.0, 0.0)];
        let mut rtc = RealToComplexNode::new();
        rtc.process(&real, &mut complex);
        assert_eq!(&complex, &expected);
    }

    #[test]
    fn test_complex_to_real() {
        let complex = vec![Complex32::new(1.0, 0.0), 
                           Complex32::new(2.0, 0.0),
                           Complex32::new(3.0, 0.0),
                           Complex32::new(4.0, 0.0)];
        let mut real = vec![0.0; 4];
        let expected = vec![1.0, 2.0, 3.0, 4.0];
        let mut ctr = ComplexToRealNode::new();
        ctr.process(&complex, &mut real);
        assert_eq!(&real, &expected);
    }

}
