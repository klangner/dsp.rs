//! Signal generators
//! 
//! Signal generators are used to generate different potentially infinite signals
//! Most generators have state and can fill buffer with data.
//! 

use std::f32;
use std::f32::consts::PI;
use rand::distributions::{Normal, Distribution};
use crate::{RealBuffer, SourceNode};


/// Impulse signal
/// x[n] = 1 if n == impulse_pos
/// x[n] = 0 if n != impulse_pos
/// 
/// Example
/// 
/// ```
/// use dsp::SourceNode;
/// use dsp::generators::ImpulseGen;
/// 
/// let mut gen = ImpulseGen::new(3);
/// let mut buffer = vec![0.0; 5];
/// gen.next(&mut buffer);
/// assert_eq!(buffer, vec![0.0, 0.0, 0.0, 1.0, 0.0]);
/// ```
pub struct ImpulseGen {
    current_sample: i64, 
    impulse_pos: i64,
}

impl ImpulseGen {
    /// Create new Impulse generator.
    ///   * impulse_pos - Impulse position
    pub fn new(impulse_pos: i64) -> ImpulseGen {
        ImpulseGen { current_sample : 0, impulse_pos }
    }
}

impl SourceNode for ImpulseGen {

    fn next(&mut self, buffer: &mut RealBuffer) -> usize {
        for sample in buffer.iter_mut() {                        
            *sample = if self.current_sample == self.impulse_pos { 1.0 } else { 0.0 };
            self.current_sample += 1;
        }
        buffer.len()
    }
}

/// Step signal
/// x[n] = 1 if n > step_pos
/// x[n] = 0 if n < step_pos
/// 
/// Example
/// 
/// ```
/// use dsp::SourceNode;
/// use dsp::generators::StepGen;
/// 
/// let mut gen = StepGen::new(3);
/// let mut buffer = vec![0.0; 5];
/// gen.next(&mut buffer);
/// assert_eq!(buffer, vec![0.0, 0.0, 0.0, 1.0, 1.0]);
/// ```
pub struct StepGen {
    current_sample: i64, 
    step_pos: i64,
}

impl StepGen {
    /// Create new Step generator 
    ///   * step_pos - Position of the step
    pub fn new(step_pos: i64) -> StepGen {
        StepGen { current_sample : 0, step_pos }
    }
}

impl SourceNode for StepGen {

    fn next(&mut self, buffer: &mut RealBuffer) -> usize {
        for sample in buffer.iter_mut() {                        
            *sample = if self.current_sample >= self.step_pos { 1.0 } else { 0.0 };
            self.current_sample += 1;
        }
        buffer.len()
    }
}

/// Sinusoidal signal
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::SourceNode;
/// use dsp::generators::SineGen;
/// 
/// let mut gen = SineGen::new(2.0, 8);
/// let mut buffer = vec![0.0; 4];
/// gen.next(&mut buffer);
/// assert_approx_eq!(buffer[0], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[1], 1.0, 1e-5f32);
/// assert_approx_eq!(buffer[2], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[3], -1.0, 1e-5f32);
/// ```
pub struct SineGen {
    current_sample: f32, 
    freq: f32,
    sample_rate: f32,
}

impl SineGen {
    /// Create new Impulse generator with impulse moved to specyfic position
    ///   * freq - Frequency in Hz
    ///   * sample_rate - How many samples per second
    pub fn new(freq: f32, sample_rate: usize) -> SineGen {
        SineGen { current_sample : 0.0, freq, sample_rate: sample_rate as f32 }
    }
}

impl SourceNode for SineGen {

    fn next(&mut self, buffer: &mut RealBuffer) -> usize {
        let w = 2.0 * PI * self.freq;
        for sample in buffer.iter_mut() {     
            let k = self.current_sample / self.sample_rate;
            *sample = f32::sin(w * k);
            self.current_sample += 1.0;
        }
        buffer.len()
    }
}

/// Generate triangular signal
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::SourceNode;
/// use dsp::generators::TriangleGen;
/// 
/// let mut gen = TriangleGen::new(2.0, 8);
/// let mut buffer = vec![0.0; 5];
/// gen.next(&mut buffer);
/// assert_approx_eq!(buffer[0], -1.0, 1e-5f32);
/// assert_approx_eq!(buffer[1], -0.5, 1e-5f32);
/// assert_approx_eq!(buffer[2], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[3], 0.5, 1e-5f32);
/// assert_approx_eq!(buffer[4], -1.0, 1e-5f32);
/// ```
pub struct TriangleGen {
    current_sample: f32, 
    freq: f32,
    sample_rate: f32,
}

impl TriangleGen {
    /// Create new Traingle generator
    ///   * freq - Frequency in Hz
    ///   * sample_rate - How many samples per second
    pub fn new(freq: f32, sample_rate: usize) -> TriangleGen {
        TriangleGen { current_sample : 0.0, freq, sample_rate: sample_rate as f32}
    }
}

impl SourceNode for TriangleGen {

    fn next(&mut self, buffer: &mut RealBuffer) -> usize {
        let w = self.sample_rate / self.freq;
        for sample in buffer.iter_mut() {     
            let k = (self.current_sample / w).fract();
            *sample = 2.0 * k - 1.0;
            self.current_sample += 1.0;
        }
        buffer.len()
    }
}


/// Generate square signal
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::SourceNode;
/// use dsp::generators::SquareGen;
/// 
/// let mut gen = SquareGen::new(2.0, 8);
/// let mut buffer = vec![0.0; 5];
/// gen.next(&mut buffer);
/// assert_approx_eq!(buffer[0], 1.0, 1e-5f32);
/// assert_approx_eq!(buffer[1], 1.0, 1e-5f32);
/// assert_approx_eq!(buffer[2], -1.0, 1e-5f32);
/// assert_approx_eq!(buffer[3], -1.0, 1e-5f32);
/// assert_approx_eq!(buffer[4], 1.0, 1e-5f32);
/// ```
pub struct SquareGen {
    current_sample: f32, 
    freq: f32,
    sample_rate: f32,
}

impl SquareGen {
    /// Create square signal
    ///   * freq - Frequency in Hz
    ///   * sample_rate - How many samples per second
    pub fn new(freq: f32, sample_rate: usize) -> SquareGen {
        SquareGen { current_sample : 0.0, freq, sample_rate: sample_rate as f32}
    }
}

impl SourceNode for SquareGen {

    fn next(&mut self, buffer: &mut RealBuffer) -> usize {
        let w = self.sample_rate / self.freq;
        for sample in buffer.iter_mut() {     
            let k = (self.current_sample / w).fract();
            *sample = if k < 0.5 { 1.0 } else { -1.0 };
            self.current_sample += 1.0;
        }
        buffer.len()
    }
}


/// Generate noise
/// 
/// Example
/// 
/// ```
/// use dsp::SourceNode;
/// use dsp::generators::NoiseGen;
/// 
/// let mut gen = NoiseGen::new(0.1);
/// let mut buffer = vec![0.0; 5];
/// gen.next(&mut buffer);
/// ```
pub struct NoiseGen {
    normal: Normal, 
}

impl NoiseGen {
    /// Create square signal
    ///   * std - Standard deviation
    pub fn new(std: f32) -> NoiseGen {
        let normal = Normal::new(0.0, std as f64);
        NoiseGen { normal }
    }
}

impl SourceNode for NoiseGen {

    fn next(&mut self, buffer: &mut RealBuffer) -> usize {
        for sample in buffer.iter_mut() {     
            *sample = self.normal.sample(&mut rand::thread_rng()) as f32;
        }
        buffer.len()
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::SourceNode;
    use super::*;

    #[test]
    fn test_impulse() {
        let mut gen = ImpulseGen::new(3);
        let mut buffer = vec![0.0; 500];
        gen.next(&mut buffer);
        assert_eq!(buffer.iter().sum::<f32>(), 1.0);
    }

}
