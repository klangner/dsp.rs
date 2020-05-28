//! Signal generators
//! 
//! Signal generators are used to generate different potentially infinite signals
//! Most generators have state and can fill buffer with data.
//! 

use std::f32;
use std::f32::consts::PI;
use rand::distributions::{Normal, Distribution};

use crate::signal::Signal;


/// Generate impulse signal
/// x[n] = 1 if n == impulse_pos
/// x[n] = 0 if n != impulse_pos
/// 
/// Example
/// 
/// ```
/// use dsp::generators::impulse;
/// 
/// let signal = impulse(100, 2, 100);
/// assert_eq!(signal.len(), 100);
/// assert_eq!(signal.data[0], 0.0);
/// assert_eq!(signal.data[1], 0.0);
/// assert_eq!(signal.data[2], 1.0);
/// assert_eq!(signal.data[3], 0.0);
/// ```
pub fn impulse(length: usize, impulse_pos: usize, sample_rate: usize) -> Signal {
    let data = (0..length).map(|i| if i == impulse_pos { 1.0 } else { 0.0 }).collect();
    Signal { data, sample_rate }
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
/// let signal = step(10, 2, 5);
/// assert_eq!(signal.data[0], 0.0);
/// assert_eq!(signal.data[1], 0.0);
/// assert_eq!(signal.data[2], 1.0);
/// assert_eq!(signal.data[3], 1.0);
/// ```
pub fn step(length: usize, step_pos: usize, sample_rate: usize) -> Signal {
    let data = (0..length).map(|i| if i >= step_pos { 1.0 } else { 0.0 }).collect();
    Signal { data, sample_rate }
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
/// assert_approx_eq!(signal.data[0], 0.0, 1e-5f32);
/// assert_approx_eq!(signal.data[1], 1.0, 1e-5f32);
/// assert_approx_eq!(signal.data[2], 0.0, 1e-5f32);
/// assert_approx_eq!(signal.data[3], -1.0, 1e-5f32);
/// ```
pub fn sine(length: usize, freq: f32, sample_rate: usize) -> Signal {
    let w = 2.0 * PI * freq / (sample_rate as f32);
    let data = (0..length).map(|i| f32::sin((i as f32) * w)).collect();
    Signal { data, sample_rate }
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
/// use dsp::generators::sawtooth;
/// 
/// let signal = sawtooth(16, 4.0, 16);
/// assert_approx_eq!(signal.data[0], -1.0, 1e-5f32);
/// assert_approx_eq!(signal.data[1], -0.5, 1e-5f32);
/// assert_approx_eq!(signal.data[2], 0.0, 1e-5f32);
/// assert_approx_eq!(signal.data[3], 0.5, 1e-5f32);
/// assert_approx_eq!(signal.data[4], -1.0, 1e-5f32);
/// ```
pub fn sawtooth(length: usize, freq: f32, sample_rate: usize) -> Signal {
    let data = (0..length).map(|i| 2.0 * ((i as f32) * freq / (sample_rate as f32)).fract() - 1.0).collect();
    Signal { data, sample_rate }
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
/// assert_approx_eq!(signal.data[0], 1.0, 1e-5f32);
/// assert_approx_eq!(signal.data[1], 1.0, 1e-5f32);
/// assert_approx_eq!(signal.data[2], -1.0, 1e-5f32);
/// assert_approx_eq!(signal.data[3], -1.0, 1e-5f32);
/// assert_approx_eq!(signal.data[4], 1.0, 1e-5f32);
/// ```
pub fn square(length: usize, freq: f32, sample_rate: usize) -> Signal {
    let data = (0..length)
        .map(|i| if ((i as f32) * freq/(sample_rate as f32)).fract() < 0.5 {1.0} else {-1.0})
        .collect();
    Signal { data, sample_rate }
}


/// Generate noise
/// 
/// Example
/// 
/// ```
/// use dsp::generators::noise;
/// 
/// let signal = noise(10, 0.1, 10);
/// ```
pub fn noise(length: usize, std: f32, sample_rate: usize) -> Signal {
    let normal = Normal::new(0.0, std as f64);
    let data = (0..length).map(|_| normal.sample(&mut rand::thread_rng()) as f32).collect();
    Signal { data, sample_rate }
}


/// A chirp is a signal in which frequency increases with time.
/// Based on:
/// https://en.wikipedia.org/wiki/Chirp#Linear
/// Create chirp signal
///   * length - in samples
///   * start_freq - Start frequency in Hz
///   * end_freq - End frequency in Hz
///   * sample_rate - Number of samples/s
pub fn chirp(length: usize, start_freq: f32, end_freq: f32, sample_rate: usize) -> Signal {
    let sweep_time = length as f32 / sample_rate as f32;

    fn sample(t: f32, start_freq: f32, end_freq: f32, sweep_time: f32) -> f32 {
        let c = (end_freq - start_freq) / sweep_time;
        let w = 2.0 * PI * (c/2.0*t.powi(2) + start_freq*t);
        f32::sin(w)
    }

    let data = (0..length)
        .map(|i| sweep_time * i as f32 / length as f32)
        .map(|t|  sample(t, start_freq, end_freq, sweep_time))
        .collect();
    Signal { data, sample_rate }
}


pub struct Sine { freq: f32, amplitude: f32 }

impl Sine {
    pub fn new(freq: f32, amplitude: f32) -> Sine {
        Sine { freq, amplitude }
    }
}

/// Create signal as a synthesis of multiple sine signals with different amplitude
/// 
/// Example
/// 
/// ```
/// use dsp::fft::ForwardFFT;
/// use dsp::generators::{synth, Sine};
/// 
/// let sines = vec![Sine::new(20.0, 2.0), Sine::new(50.0, 0.5)];
/// let signal = synth(1024, sines, 512);
/// let mut ft = ForwardFFT::new(1024);
/// let spectrum = ft.process(&signal);
/// assert_eq!(spectrum.max_freq(), 20.0);
/// ```
pub fn synth(length: usize, gens: Vec<Sine>, sample_rate: usize) -> Signal {
    if gens.is_empty() {
        Signal::empty(sample_rate)
    } else {
        let norm: f32 = gens.len() as f32;
        let mut buffer = sine(length, gens[0].freq, sample_rate)
                            .rescale(gens[0].amplitude / norm)
                            .data
                            .to_owned();
        for g in &gens[1..] {
            let signal = sine(length, g.freq, sample_rate).rescale(g.amplitude / norm);
            for i in 0..length {
                buffer[i] += signal.data[i];
            }
        }
        Signal::new(buffer, sample_rate)
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
}
