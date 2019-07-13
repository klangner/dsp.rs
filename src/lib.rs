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


// impl RealBuffer {
//     pub fn to_complex(&self, output: &mut ComplexBuffer) {
//         let n = usize::min(self.data.len(), output.len());
//         for i in 0..n {
//             output[i] = Complex32::new(self.data[i], 0.0);
//         }
//     }
// }

// impl ComplexBuffer {
//     pub fn to_real(&self, output: &mut RealBuffer) {
//         let n = usize::min(self.data.len(), output.len());
//         for i in 0..n {
//             output[i] = self[i].norm();
//         }
//     }
// }


trait SourceNode {
    type Buffer;
    
    fn next_batch(&mut self, output: &mut Self::Buffer);
}

trait ProcessingNode {
    type InBuffer;
    type OutBuffer;
    
    fn next(input: &Self::InBuffer, output: &mut Self::OutBuffer);
}

trait SinkNode {
    type Buffer;
    
    fn next(input: &Self::Buffer);
}




