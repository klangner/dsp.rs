//! Signal generator
//! 
//! Signal generator are used to generate different infinite signals
//! Most generator have state and can fill buffer with data.
//! 

use std::f32;
use std::f32::consts::PI;
use anyhow::Result;
#[cfg(feature = "random")]
use rand;
#[cfg(feature = "random")]
use rand_distr::{Normal, Distribution};

use crate::runtime::node::SourceNode;


/// Generate impulse signal
/// x[n] = 1 if n == impulse_pos
/// x[n] = 0 if n != impulse_pos
/// 
/// Example
/// 
/// ```
/// use dsp::runtime::node::SourceNode;
/// use dsp::node::generator::Impulse;
/// 
/// let mut signal = Impulse::new();
/// let mut buffer = vec![2.0;100];
/// let _ = signal.write_buffer(&mut buffer);
/// 
/// assert_eq!(buffer[0], 1.0);
/// assert_eq!(buffer[1], 0.0);
/// assert_eq!(buffer[2], 0.0);
/// ```
pub struct Impulse {
     impulse_send: bool,
}

impl Impulse {
    pub fn new() -> Impulse {
        Impulse {impulse_send: false}
    }
}

impl SourceNode<f32> for Impulse {
    fn write_buffer(&mut self, buffer: &mut [f32]) -> Result<()> {
        for e in buffer.iter_mut() {*e = 0.};
        if !self.impulse_send && buffer.len() > 0 {
            buffer[0] = 1.;
            self.impulse_send = true;
        }
        Ok(())
    }
}


/// Step signal
/// x[n] = 1 if n > step_pos
/// x[n] = 0 if n < step_pos
/// 
/// Example
/// 
/// ```
/// use dsp::runtime::node::SourceNode;
/// use dsp::node::generator::Step;
/// 
/// let mut signal = Step::new(2);
/// let mut buffer = vec![2.0;100];
/// let _ = signal.write_buffer(&mut buffer);
/// 
/// assert_eq!(buffer[0], 0.0);
/// assert_eq!(buffer[1], 0.0);
/// assert_eq!(buffer[2], 1.0);
/// assert_eq!(buffer[3], 1.0);
/// ```
pub struct Step {
    step_pos: usize,
}

impl Step {
    pub fn new(step_pos: usize) -> Step {
        Step{ step_pos }
    }
}

impl SourceNode<f32> for Step {
    fn write_buffer(&mut self, buffer: &mut [f32]) -> Result<()> {
        for i in 0..buffer.len() {
            if self.step_pos  > 0 {
                buffer[i] = 0.;
                self.step_pos -= 1;
            } else {
                buffer[i] = 1.;
            }
        }
        Ok(())
    }
}

/// Sinusoidal signal
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::runtime::node::SourceNode;
/// use dsp::node::generator::Sinusoid;
/// 
/// let mut signal = Sinusoid::new(2.0, 8);
/// let mut buffer = vec![0.0;10];
/// let _ = signal.write_buffer(&mut buffer);
/// 
/// assert_approx_eq!(buffer[0], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[1], 1.0, 1e-5f32);
/// assert_approx_eq!(buffer[2], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[3], -1.0, 1e-5f32);
/// ```
pub struct Sinusoid {
    step_pos: usize,
    freq: f32,
    sample_rate: usize,
}

impl Sinusoid {
    /// Create new sinusoid generator
    ///   * freq - signal frequency
    ///   * sample_rate - Number of samples/s
    pub fn new(freq: f32, sample_rate: usize) -> Sinusoid {
        Sinusoid { step_pos: 0, freq, sample_rate}
    }
}

impl SourceNode<f32> for Sinusoid {
    fn write_buffer(&mut self, buffer: &mut [f32]) -> Result<()> {
        let w = 2.0 * PI * self.freq / (self.sample_rate as f32);
        for i in 0..buffer.len() {
            buffer[i] = f32::sin((self.step_pos as f32) * w);
            self.step_pos += 1;
            if self.step_pos >= self.sample_rate {
                self.step_pos = 0;
            }
        }
        Ok(())
    }
}

/// Generate triangular signal
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::runtime::node::SourceNode;
/// use dsp::node::generator::Sawtooth;
/// 
/// let mut signal = Sawtooth::new(4.0, 16);
/// let mut buffer = vec![0.0;10];
/// let _ = signal.write_buffer(&mut buffer);
/// 
/// assert_approx_eq!(buffer[0], -1.0, 1e-5f32);
/// assert_approx_eq!(buffer[1], -0.5, 1e-5f32);
/// assert_approx_eq!(buffer[2], 0.0, 1e-5f32);
/// assert_approx_eq!(buffer[3], 0.5, 1e-5f32);
/// assert_approx_eq!(buffer[4], -1.0, 1e-5f32);
/// ```
pub struct Sawtooth {
    step_pos: usize,
    freq: f32,
    sample_rate: usize,
}

impl Sawtooth {
    /// Create new Triangle generator
    ///   * freq - signal frequency
    ///   * sample_rate - Number of samples/s
    pub fn new(freq: f32, sample_rate: usize) -> Sawtooth {
        Sawtooth { step_pos: 0, freq, sample_rate}
    }
}

impl SourceNode<f32> for Sawtooth {
    fn write_buffer(&mut self, buffer: &mut [f32]) -> Result<()> {
        for i in 0..buffer.len() {
            let value = 2.0 * ((self.step_pos as f32) * self.freq / (self.sample_rate as f32)).fract() - 1.0;
            buffer[i] = value;
            self.step_pos += 1;
            if self.step_pos >= self.sample_rate {
                self.step_pos = 0;
            }
        }
        Ok(())
    }
}


/// Generate square signal
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::runtime::node::SourceNode;
/// use dsp::node::generator::Square;
/// 
/// let mut signal = Square::new(4.0, 16);
/// let mut buffer = vec![0.0;10];
/// let _ = signal.write_buffer(&mut buffer);
/// 
/// assert_approx_eq!(buffer[0], 1.0, 1e-5f32);
/// assert_approx_eq!(buffer[1], 1.0, 1e-5f32);
/// assert_approx_eq!(buffer[2], -1.0, 1e-5f32);
/// assert_approx_eq!(buffer[3], -1.0, 1e-5f32);
/// assert_approx_eq!(buffer[4], 1.0, 1e-5f32);
/// ```
pub struct Square {
    step_pos: usize,
    freq: f32,
    sample_rate: usize,
}

impl Square {
    /// Create new square function generator
    ///   * freq - signal frequency
    ///   * sample_rate - Number of samples/s
    pub fn new(freq: f32, sample_rate: usize) -> Square {
        Square { step_pos: 0, freq, sample_rate}
    }
}

impl SourceNode<f32> for Square {
    fn write_buffer(&mut self, buffer: &mut [f32]) -> Result<()> {
        for i in 0..buffer.len() {
            let value = if ((self.step_pos as f32) * self.freq/(self.sample_rate as f32)).fract() < 0.5 {
                1.0
            } else {
                -1.0
            };
            buffer[i] = value;
            self.step_pos += 1;
            if self.step_pos >= self.sample_rate {
                self.step_pos = 0;
            }
        }
        Ok(())
    }
}


/// Generate noise
/// 
/// Example
/// 
/// ```
/// use dsp::runtime::node::SourceNode;
/// use dsp::node::generator::Noise;
/// 
/// let mut signal = Noise::new(4.0);
/// let mut buffer = vec![0.0;10];
/// let _ = signal.write_buffer(&mut buffer);
/// ```
#[cfg(feature = "random")]
pub struct Noise {
    std: f32,
}

#[cfg(feature = "random")]
impl Noise {
    pub fn new(std: f32) -> Noise {
        Noise { std }
    }
}

#[cfg(feature = "random")]
impl SourceNode<f32> for Noise {
    fn write_buffer(&mut self, buffer: &mut [f32]) -> Result<()> { 
        let normal = Normal::new(0.0, self.std as f64).unwrap();
        for i in 0..buffer.len() {
            buffer[i] = normal.sample(&mut rand::thread_rng()) as f32;
        }
        Ok(())
   }
}


/// A chirp is a signal in which frequency increases with time.
/// Based on:
/// https://en.wikipedia.org/wiki/Chirp#Linear
/// 
pub struct Chirp {
    sweep_time: f32, 
    start_freq: f32, 
    end_freq: f32, 
    sample_rate: usize,
    sample_pos: usize
}

impl Chirp {
    /// Create chirp signal
    ///   * length - in seconds
    ///   * start_freq - Start frequency in Hz
    ///   * end_freq - End frequency in Hz
    ///   * sample_rate - Number of samples/s
    pub fn new(sweep_time: f32, start_freq: f32, end_freq: f32, sample_rate: usize) -> Chirp {
        Chirp {sweep_time, start_freq, end_freq, sample_rate, sample_pos: 0}
    }
        
    fn sample(&self, t: f32) -> f32 {
        let c = (self.end_freq - self.start_freq) / self.sweep_time;
        let w = 2.0 * PI * (c/2.0*t.powi(2) + self.start_freq*t);
        f32::sin(w)
    }
}

impl SourceNode<f32> for Chirp {
    fn write_buffer(&mut self, buffer: &mut [f32]) -> Result<()> { 
        for i in 0..buffer.len() {
            let t = (self.sample_pos + i) as f32 / self.sample_rate as f32;
            buffer[i] = self.sample(t);
            if t <= self.sweep_time { 
                self.sample_pos += 1
            }
        }
        Ok(())
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use crate::runtime::node::SourceNode;
    use crate::node::generator::Sinusoid;

    #[test]
    fn test_sine_small_buffer() {
        let mut signal = Sinusoid::new(2.0, 8);
        let mut buffer = vec![0.0;3];
        let _ = signal.write_buffer(&mut buffer);

        assert_approx_eq!(buffer[0], 0.0, 1e-5f32);
        assert_approx_eq!(buffer[1], 1.0, 1e-5f32);
        assert_approx_eq!(buffer[2], 0.0, 1e-5f32);
        
        let _ = signal.write_buffer(&mut buffer);
        assert_approx_eq!(buffer[0], -1.0, 1e-5f32);       
        assert_approx_eq!(buffer[1], 0.0, 1e-5f32);
        assert_approx_eq!(buffer[2], 1.0, 1e-5f32);
    }

    #[test]
    fn test_sine_large_buffer() {
        let mut signal = Sinusoid::new(2.0, 8);
        let mut buffer = vec![0.0;10];
        let _ = signal.write_buffer(&mut buffer);
        let _ = signal.write_buffer(&mut buffer);
        assert_approx_eq!(buffer[0], 0.0, 1e-5f32);
        assert_approx_eq!(buffer[1], -1.0, 1e-5f32);       
        assert_approx_eq!(buffer[2], 0.0, 1e-5f32);
        assert_approx_eq!(buffer[3], 1.0, 1e-5f32);
    }
}