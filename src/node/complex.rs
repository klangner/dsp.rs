//! Node for converting between real and complex numbers
//! 
//! 

use anyhow::Result;
use crate::num_complex::Complex32;
use crate::runtime::node::ProcessNode;


/// Implement Real -> complex converter
/// 
/// Example
/// 
/// ```
/// use dsp::runtime::node::ProcessNode;
/// use dsp::node::complex::RealToComplex;
/// use dsp::num_complex::Complex32;
/// 
/// let node = RealToComplex::new();
/// let input_buffer = vec![3.0;10];
/// let mut output_buffer = vec![Complex32::new(0., 0.);10];
/// node.process_buffer(&input_buffer, &mut output_buffer);
/// 
/// assert_eq!(output_buffer[0], Complex32::new(3., 0.));
/// assert_eq!(output_buffer[1], Complex32::new(3., 0.));
/// ```

pub struct RealToComplex {}

impl RealToComplex {
    pub fn new() -> RealToComplex {
        RealToComplex {}
    }
}

impl ProcessNode<f32, Complex32> for RealToComplex {
    fn process_buffer(&self, input_buffer: &[f32], output_buffer: &mut [Complex32]) -> Result<()> {
        let n = usize::min(input_buffer.len(), output_buffer.len());
        for i in 0..n {
            output_buffer[i] = Complex32::new(input_buffer[i], 0.); 
        }
        Ok(())
    }
}


/// Implement Complex -> Real converter
/// 
/// Example
/// 
/// ```
/// use dsp::runtime::node::ProcessNode;
/// use dsp::node::complex::ComplexToReal;
/// use dsp::num_complex::Complex32;
/// 
/// let node = ComplexToReal::new();
/// let input_buffer = vec![Complex32::new(4., 3.);10];
/// let mut output_buffer = vec![0.0;10];
/// node.process_buffer(&input_buffer, &mut output_buffer);
/// 
/// assert_eq!(output_buffer[0], 5.);
/// assert_eq!(output_buffer[1], 5.);
/// ```

pub struct ComplexToReal {}

impl ComplexToReal {
    pub fn new() -> ComplexToReal {
        ComplexToReal {}
    }
}

impl ProcessNode<Complex32, f32> for ComplexToReal {
    fn process_buffer(&self, input_buffer: &[Complex32], output_buffer: &mut [f32]) -> Result<()> {
        let n = usize::min(input_buffer.len(), output_buffer.len());
        for i in 0..n {
            output_buffer[i] = input_buffer[i].norm(); 
        }
        Ok(())
    }
}