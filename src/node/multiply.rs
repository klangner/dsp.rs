//! Node for multiplying signal samples by constant value
//! 

use anyhow::Result;
use crate::runtime::node::ProcessNode;


/// Multiply buffer sample by constant value
/// 
/// Example
/// 
/// ```
/// use dsp::runtime::node::ProcessNode;
/// use dsp::node::multiply::MultiplyConst;
/// 
/// let node = MultiplyConst::new(3.);
/// let input_buffer = vec![1., 2., 3.];
/// let mut output_buffer = vec![0.;3];
/// node.process_buffer(&input_buffer, &mut output_buffer);
/// 
/// assert_eq!(output_buffer[0], 3.);
/// assert_eq!(output_buffer[1], 6.);
/// assert_eq!(output_buffer[2], 9.);
/// ```
pub struct MultiplyConst {
    value: f32,
}

impl MultiplyConst {
    pub fn new(value: f32) -> MultiplyConst {
        MultiplyConst {value}
    }
}

impl ProcessNode<f32, f32> for MultiplyConst {
    fn process_buffer(&self, input_buffer: &[f32], output_buffer: &mut [f32]) -> Result<()> {
        let n = usize::min(input_buffer.len(), output_buffer.len());
        for i in 0..n {
            output_buffer[i] = self.value * input_buffer[i]; 
        }
        Ok(())
    }
}
