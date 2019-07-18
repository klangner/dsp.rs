//! Signal generators
//! 
//! Signal generators are used to generate different potentially infinite signals
//! Most generators have state and can fill buffer with data.
//! 

use std::f32;
use std::f32::consts::PI;
use rand::distributions::{Normal, Distribution};

use crate::{RealBuffer, SourceNode};


/// This trait is implemented by node which is used to generate signals
pub trait SignalGen {

    /// Generate nth sample.
    fn sample(&self, t: f32) -> f32;

    /// Function for checking if generator has next frame of data
    /// Return true if it has.
    fn has_next(&self) -> bool { true }

}


/// Impulse signal
/// x[n] = 1 if n == impulse_pos
/// x[n] = 0 if n != impulse_pos
/// 
/// Example
/// 
/// ```
/// use dsp::generators::{SignalGen, ImpulseGen};
/// 
/// let mut gen = ImpulseGen::new(0.2);
/// assert_eq!(gen.sample(0.0), 0.0);
/// assert_eq!(gen.sample(0.1), 0.0);
/// assert_eq!(gen.sample(0.2), 1.0);
/// assert_eq!(gen.sample(0.3), 0.0);
/// ```
pub struct ImpulseGen {
    impulse_pos: f32,
}

impl ImpulseGen {
    /// Create new Impulse generator.
    ///   * impulse_pos - Impulse position
    pub fn new(impulse_pos: f32) -> ImpulseGen {
        ImpulseGen { impulse_pos }
    }
}

impl SignalGen for ImpulseGen {

    fn sample(&self, t: f32) -> f32 {
        if t == self.impulse_pos { 1.0 } else { 0.0 }
    }
}

/// Step signal
/// x[n] = 1 if n > step_pos
/// x[n] = 0 if n < step_pos
/// 
/// Example
/// 
/// ```
/// use dsp::generators::{SignalGen, StepGen};
/// 
/// let mut gen = StepGen::new(0.2);
/// assert_eq!(gen.sample(0.0), 0.0);
/// assert_eq!(gen.sample(0.1), 0.0);
/// assert_eq!(gen.sample(0.2), 1.0);
/// assert_eq!(gen.sample(0.3), 1.0);
/// ```
pub struct StepGen {
    step_pos: f32,
}

impl StepGen {
    /// Create new Step generator 
    ///   * step_pos - Position of the step
    pub fn new(step_pos: f32) -> StepGen {
        StepGen { step_pos }
    }
}

impl SignalGen for StepGen {

    fn sample(&self, t: f32) -> f32 {
        if t >= self.step_pos { 1.0 } else { 0.0 }
    }
}

/// Sinusoidal signal
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::generators::{SignalGen, SineGen};
/// 
/// let mut gen = SineGen::new(2.0);
/// assert_approx_eq!(gen.sample(0.0), 0.0, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.125), 1.0, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.25), 0.0, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.375), -1.0, 1e-5f32);
/// ```
pub struct SineGen {
    freq: f32,
}

impl SineGen {
    /// Create a new Sine generator with a given frequency.
    ///   * freq - Frequency in Hz
    ///   * sample_freq - How many samples per second
    pub fn new(freq: f32) -> SineGen {
        SineGen { freq }
    }
}

impl SignalGen for SineGen {

    fn sample(&self, t: f32) -> f32 {
        f32::sin(2.0 * PI * t * self.freq)
    }
}


/// Generate triangular signal
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::generators::{SignalGen, TriangleGen};
/// 
/// let mut gen = TriangleGen::new(2.0);
/// assert_approx_eq!(gen.sample(0.0), -1.0, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.125), -0.5, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.25), 0.0, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.375), 0.5, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.5), -1.0, 1e-5f32);
/// ```
pub struct TriangleGen {
    freq: f32,
}

impl TriangleGen {
    /// Create new Traingle generator
    ///   * freq - Frequency in Hz
    ///   * sample_rate - How many samples per second
    pub fn new(freq: f32) -> TriangleGen {
        TriangleGen { freq }
    }
}

impl SignalGen for TriangleGen {

    fn sample(&self, t: f32) -> f32 {
        let k = (t * self.freq).fract();
        2.0 * k - 1.0
    }
}


/// Generate square signal
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::generators::{SignalGen, SquareGen};
/// 
/// let mut gen = SquareGen::new(2.0);
/// assert_approx_eq!(gen.sample(0.0), 1.0, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.125), 1.0, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.25), -1.0, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.375), -1.0, 1e-5f32);
/// assert_approx_eq!(gen.sample(0.5), 1.0, 1e-5f32);
/// ```
pub struct SquareGen {
    freq: f32,
}

impl SquareGen {
    /// Create square signal
    ///   * freq - Frequency in Hz
    ///   * sample_rate - How many samples per second
    pub fn new(freq: f32) -> SquareGen {
        SquareGen { freq }
    }
}

impl SignalGen for SquareGen {

    fn sample(&self, t: f32) -> f32 {
        let k = (t * self.freq).fract();
        if k < 0.5 { 1.0 } else { -1.0 }
    }
}


/// Generate noise
/// 
/// Example
/// 
/// ```
/// use dsp::generators::{SignalGen, NoiseGen};
/// 
/// let mut gen = NoiseGen::new(0.1);
/// gen.sample(0.0);
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

impl SignalGen for NoiseGen {

    fn sample(&self, _t: f32) -> f32 {
        self.normal.sample(&mut rand::thread_rng()) as f32
    }
}


/// A chirp is a signal in which frequency increases with time.
/// Based on:
/// https://en.wikipedia.org/wiki/Chirp#Linear
pub struct ChirpGen {
    start_freq: f32,
    end_freq: f32,
    sweep_time: f32,
}

impl ChirpGen {
    /// Create chirp signal
    ///   * start_freq - Start frequency in Hz
    ///   * end_freq - End frequency in Hz
    ///   * length - in seconds
    ///   * sample_rate - How many samples per second
    pub fn new(start_freq: f32, end_freq: f32, sweep_time: f32) -> ChirpGen {
        ChirpGen { start_freq, end_freq, sweep_time }
    }
}

impl SignalGen for ChirpGen {

    fn sample(&self, t: f32) -> f32 {
        if t > self.sweep_time {
            0.0
        } else {
            let c = (self.end_freq - self.start_freq) / (self.sweep_time);
            let w = 2.0 * PI * (c/2.0*t.powi(2) + self.start_freq*t);
            f32::sin(w)
        }
    }
}

/// Create Source node based on generator
pub struct GenNode {
    gen: Box<SignalGen>,
    output: RealBuffer,
    sample_freq: f32,
    current_sample: f32,
}

impl GenNode {
    pub fn new(gen: Box<SignalGen>, sample_freq: f32, buffer_size: usize) -> GenNode {
        GenNode { gen, output: vec![0.0; buffer_size], sample_freq, current_sample: 0.0 }
    }
}

impl SourceNode for GenNode {
    type Buffer = RealBuffer;
    
    fn next_frame(&mut self) -> &RealBuffer {
        for i in self.output.iter_mut() {
            *i = self.gen.sample(self.current_sample / self.sample_freq);
            self.current_sample += 1.0;
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

    #[test]
    fn test_impulse() {
        let gen = ImpulseGen::new(0.2);
        assert_eq!(gen.sample(0.0), 0.0);
        assert_eq!(gen.sample(0.1), 0.0);
        assert_eq!(gen.sample(0.2), 1.0);
        assert_eq!(gen.sample(0.3), 0.0);
    }
    
    #[test]
    fn test_gen_node() {
        let mut node = GenNode::new(Box::new(ImpulseGen::new(0.2)), 10.0, 4);
        let output = node.next_frame();
        let expected = vec![0.0, 0.0, 1.0, 0.0];
        assert_eq!(output, &expected);
    }
}
