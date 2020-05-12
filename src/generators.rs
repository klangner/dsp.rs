//! Signal generators
//! 
//! Signal generators are used to generate different potentially infinite signals
//! Most generators have state and can fill buffer with data.
//! 

use std::f32;
use std::f32::consts::PI;
use rand::distributions::{Normal, Distribution};

use crate::RealBuffer;


/// Generate impulse signal
/// x[n] = 1 if n == impulse_pos
/// x[n] = 0 if n != impulse_pos
/// 
/// Example
/// 
/// ```
/// use dsp::generators::impulse;
/// 
/// let signal = impulse(100, 2);
/// assert_eq!(signal.len(), 100);
/// assert_eq!(signal[0], 0.0);
/// assert_eq!(signal[1], 0.0);
/// assert_eq!(signal[2], 1.0);
/// assert_eq!(signal[3], 0.0);
/// ```
pub fn impulse(length: usize, impulse_pos: usize) -> RealBuffer {
    (0..length).map(|i| if i == impulse_pos { 1.0 } else { 0.0 }).collect()
}


/// Step signal
/// x[n] = 1 if n > step_pos
/// x[n] = 0 if n < step_pos
/// 
/// Example
/// 
/// ```
/// use dsp::generators::step;
/// 
/// let signal = step(10, 2);
/// assert_eq!(signal[0], 0.0);
/// assert_eq!(signal[1], 0.0);
/// assert_eq!(signal[2], 1.0);
/// assert_eq!(signal[3], 1.0);
/// ```
pub fn step(length: usize, step_pos: usize) -> RealBuffer {
    (0..length).map(|i| if i >= step_pos { 1.0 } else { 0.0 }).collect()
}


/// Sinusoidal signal
///   * length - size of the output vector
///   * freq - signal frequency
///   * sample_rate - Number of samples/s
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::generators::sine;
/// 
/// let signal = sine(10, 2.0, 8);
/// assert_approx_eq!(signal[0], 0.0, 1e-5f32);
/// assert_approx_eq!(signal[1], 1.0, 1e-5f32);
/// assert_approx_eq!(signal[2], 0.0, 1e-5f32);
/// assert_approx_eq!(signal[3], -1.0, 1e-5f32);
/// ```
pub fn sine(length: usize, freq: f32, sample_rate: usize) -> RealBuffer {
    let w = 2.0 * PI * freq / (sample_rate as f32);
   (0..length).map(|i| f32::sin((i as f32) * w)).collect()
}


/// Generate triangular signal
///   * length - size of the output vector
///   * freq - signal frequency
///   * sample_rate - Number of samples/s
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::generators::traingle;
/// 
/// let signal = traingle(16, 4.0, 16);
/// assert_approx_eq!(signal[0], -1.0, 1e-5f32);
/// assert_approx_eq!(signal[1], -0.5, 1e-5f32);
/// assert_approx_eq!(signal[2], 0.0, 1e-5f32);
/// assert_approx_eq!(signal[3], 0.5, 1e-5f32);
/// assert_approx_eq!(signal[4], -1.0, 1e-5f32);
/// ```
pub fn traingle(length: usize, freq: f32, sample_rate: usize) -> RealBuffer {
    (0..length).map(|i| 2.0 * ((i as f32) * freq / (sample_rate as f32)).fract() - 1.0).collect()
}


/// Generate square signal
///   * length - size of the output vector
///   * signal frequency
///   * Number of samples/s
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::generators::square;
/// 
/// let signal = square(10, 4.0, 16);
/// assert_approx_eq!(signal[0], 1.0, 1e-5f32);
/// assert_approx_eq!(signal[1], 1.0, 1e-5f32);
/// assert_approx_eq!(signal[2], -1.0, 1e-5f32);
/// assert_approx_eq!(signal[3], -1.0, 1e-5f32);
/// assert_approx_eq!(signal[4], 1.0, 1e-5f32);
/// ```
pub fn square(length: usize, freq: f32, sample_rate: usize) -> RealBuffer {
    (0..length)
        .map(|i| if ((i as f32) * freq/(sample_rate as f32)).fract() < 0.5 {1.0} else {-1.0})
        .collect()
}


/// Generate noise
/// 
/// Example
/// 
/// ```
/// use dsp::generators::noise;
/// 
/// let signal = noise(10, 0.1);
/// ```
pub fn noise(length: usize, std: f32) -> RealBuffer {
    let normal = Normal::new(0.0, std as f64);
    (0..length).map(|_| normal.sample(&mut rand::thread_rng()) as f32).collect()
}


/// A chirp is a signal in which frequency increases with time.
/// Based on:
/// https://en.wikipedia.org/wiki/Chirp#Linear
/// Create chirp signal
///   * length - in samples
///   * start_freq - Start frequency in Hz
///   * end_freq - End frequency in Hz
///   * sample_rate - Number of samples/s
pub fn chirp(length: usize, start_freq: f32, end_freq: f32, sample_rate: usize) -> RealBuffer {
    let sweep_time = length as f32 / sample_rate as f32;
    fn sample(t: f32, start_freq: f32, end_freq: f32, sweep_time: f32) -> f32 {
        let c = (end_freq - start_freq) / sweep_time;
        let w = 2.0 * PI * (c/2.0*t.powi(2) + start_freq*t);
        f32::sin(w)
    }

    (0..length)
        .map(|i| sweep_time * i as f32 / length as f32)
        .map(|t|  sample(t, start_freq, end_freq, sweep_time))
        .collect()
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
}
