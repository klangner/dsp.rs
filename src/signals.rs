
use crate::{RealBuffer, ProcessingNode};


/// Rescale signal amplitude
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;    
/// use dsp::{ProcessingNode, SourceNode};
/// use dsp::generators::{SineGen, GenNode};
/// use dsp::signals::GainNode;
/// 
/// let mut gen = GenNode::new(Box::new(SineGen::new(1.0)), 4.0, 4);
/// let mut amplitude_node = GainNode::new(2.0, 4);
/// let signal = gen.next_frame();
/// let scaled_signal = amplitude_node.process(signal);
/// assert_approx_eq!(scaled_signal[0], 0.0, 1e-5f32);
/// assert_approx_eq!(scaled_signal[1], 2.0, 1e-5f32);
/// assert_approx_eq!(scaled_signal[2], 0.0, 1e-5f32);
/// assert_approx_eq!(scaled_signal[3], -2.0, 1e-5f32);
/// ```
pub struct GainNode {
    scale: f32,
    output: RealBuffer,
}

impl GainNode {
    pub fn new(scale: f32, frame_size: usize) -> GainNode {
        GainNode { scale, output: vec![0.0; frame_size] }
    }
}

impl ProcessingNode for GainNode {
    type InBuffer = RealBuffer;
    type OutBuffer = RealBuffer;
    
    fn process(&mut self, input: &RealBuffer) -> &RealBuffer {
        let n = usize::min(input.len(), self.output.len());
        for i in 0..n {
            self.output[i] = self.scale * input[i];
        }
        &self.output
    }

}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;    
    use crate::{ProcessingNode, SourceNode};
    use crate::generators::{SineGen, GenNode};
    use super::*;

    #[test]
    fn test_gen_node() {
        let mut gen = GenNode::new(Box::new(SineGen::new(1.0)), 4.0, 4);
        let mut amplitude_node = GainNode::new(2.0, 4);
        let signal = gen.next_frame();
        let scaled_signal = amplitude_node.process(signal);
        assert_approx_eq!(scaled_signal[0], 0.0, 1e-5f32);
        assert_approx_eq!(scaled_signal[1], 2.0, 1e-5f32);
        assert_approx_eq!(scaled_signal[2], 0.0, 1e-5f32);
        assert_approx_eq!(scaled_signal[3], -2.0, 1e-5f32);
    }
}