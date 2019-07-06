//! Signal generators
//! 
//! Signal generators are used to generate different potentially infinite signals
//! Most generators have state and can fill buffer with data.
//! 

use std::f32;
use std::f32::consts::PI;
use rand::distributions::{Normal, Distribution};


/// This trait is implemented by node which is used to generate signals
pub trait SignalGen {

    /// Generate next sample.
    fn next(&mut self) -> f32;

    /// Add listener to this generator
    // fn add_listener(&self, f: Fn(f32) -> ());

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
/// let mut gen = ImpulseGen::new(2);
/// assert_eq!(gen.next(), 0.0);
/// assert_eq!(gen.next(), 0.0);
/// assert_eq!(gen.next(), 1.0);
/// assert_eq!(gen.next(), 0.0);
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

impl SignalGen for ImpulseGen {

    fn next(&mut self) -> f32 {
        let sample = if self.current_sample == self.impulse_pos { 1.0 } else { 0.0 };
        self.current_sample += 1;
        sample
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
/// let mut gen = StepGen::new(2);
/// assert_eq!(gen.next(), 0.0);
/// assert_eq!(gen.next(), 0.0);
/// assert_eq!(gen.next(), 1.0);
/// assert_eq!(gen.next(), 1.0);
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

impl SignalGen for StepGen {

    fn next(&mut self) -> f32 {
        let sample = if self.current_sample >= self.step_pos { 1.0 } else { 0.0 };
        self.current_sample += 1;
        sample
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
/// let mut gen = SineGen::new(2.0, 8);
/// assert_approx_eq!(gen.next(), 0.0, 1e-5f32);
/// assert_approx_eq!(gen.next(), 1.0, 1e-5f32);
/// assert_approx_eq!(gen.next(), 0.0, 1e-5f32);
/// assert_approx_eq!(gen.next(), -1.0, 1e-5f32);
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

impl SignalGen for SineGen {

    fn next(&mut self) -> f32 {
        let w = 2.0 * PI * self.freq;
        let sample = f32::sin(w * self.current_sample / self.sample_rate);
        self.current_sample += 1.0;
        sample
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
/// let mut gen = TriangleGen::new(2.0, 8);
/// assert_approx_eq!(gen.next(), -1.0, 1e-5f32);
/// assert_approx_eq!(gen.next(), -0.5, 1e-5f32);
/// assert_approx_eq!(gen.next(), 0.0, 1e-5f32);
/// assert_approx_eq!(gen.next(), 0.5, 1e-5f32);
/// assert_approx_eq!(gen.next(), -1.0, 1e-5f32);
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

impl SignalGen for TriangleGen {

    fn next(&mut self) -> f32 {
        let w = self.sample_rate / self.freq;
        let k = (self.current_sample / w).fract();
        let sample = 2.0 * k - 1.0;
        self.current_sample += 1.0;
        sample
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
/// let mut gen = SquareGen::new(2.0, 8);
/// assert_approx_eq!(gen.next(), 1.0, 1e-5f32);
/// assert_approx_eq!(gen.next(), 1.0, 1e-5f32);
/// assert_approx_eq!(gen.next(), -1.0, 1e-5f32);
/// assert_approx_eq!(gen.next(), -1.0, 1e-5f32);
/// assert_approx_eq!(gen.next(), 1.0, 1e-5f32);
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

impl SignalGen for SquareGen {

    fn next(&mut self) -> f32 {
        let w = self.sample_rate / self.freq;
        let k = (self.current_sample / w).fract();
        let sample = if k < 0.5 { 1.0 } else { -1.0 };
        self.current_sample += 1.0;
        sample
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
/// gen.next();
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

    fn next(&mut self) -> f32 {
        self.normal.sample(&mut rand::thread_rng()) as f32
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
        let mut gen = ImpulseGen::new(2);
        assert_eq!(gen.next(), 0.0);
        assert_eq!(gen.next(), 0.0);
        assert_eq!(gen.next(), 1.0);
        assert_eq!(gen.next(), 0.0);
    }

}
