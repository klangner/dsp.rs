//! Standard Windows functions

use std::cmp;
use std::f32::consts::PI;
use crate::{RealBuffer, ProcessingNode};
use crate::vectors;


/// A window function. Can be applied to a signal
#[derive(Clone, Debug, PartialEq)]
pub struct Window {
    samples: Vec<f32>,
}

impl Window {
    /// Returns the length of the window, in frames
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Apply this window to the given frame
    pub fn apply(&self, input: &RealBuffer, mut output: &mut RealBuffer) {
        vectors::multiply(&self.samples, &input, &mut output);
    }
}


/// Window as a ProcessingNode
pub struct WindowNode {
    window: Window,
    output: RealBuffer,
}

impl WindowNode {
    pub fn new(window: Window) -> WindowNode {
        let output = vec![0.0; window.len()];
        WindowNode { window, output }
    }
}

impl ProcessingNode for WindowNode {
    type InBuffer = RealBuffer;
    type OutBuffer = RealBuffer;
    
    fn process(&mut self, input: &RealBuffer) -> &RealBuffer {
        self.window.apply(input, &mut self.output);
        &self.output
    }
}
/// Compute a simple rectangular window, a.k.a. __boxcar__ or __Dirichlet__ window
/// 
/// Example
/// 
/// ```
/// use dsp::windows;
/// 
/// let win = windows::rectangular(3, 1, 6);
/// let frame = vec![1.0; 6];
/// let mut output = vec![0.0; 6];
/// win.apply(&frame, &mut output);
/// assert_eq!(output, vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0]);
/// ```
pub fn rectangular(width: usize, offset: usize, window_length: usize) -> Window {
    let mut samples = vec![0.0; window_length];
    let end = cmp::min(offset + width, window_length);
    for i in offset..end {
        samples[i] = 1.0;    
    }
    Window { samples }
}

/// Creates a triangular window
/// 
/// Example
/// 
/// ```
/// use dsp::windows;
/// 
/// let win = windows::triangular(5, 1, 7);
/// let frame = vec![1.0; 7];
/// let mut output = vec![0.0; 7];
/// win.apply(&frame, &mut output);
/// assert_eq!(output, vec![0.0, 0.0, 0.5, 1.0, 0.5, 0.0, 0.0]);
/// ```
pub fn triangular(width: usize, offset: usize, window_length: usize) -> Window {
    let mut samples = vec![0.0; window_length];
    let end = cmp::min(offset + width, window_length);
    let slope = 2.0 / ((width - 1) as f32);
    for i in offset..end {
        let y = (i - offset) as f32 * slope;
        samples[i] = if i - offset < window_length / 2 { y } else { 2.0 - y }    
    }
    Window { samples }
}

/// Create the Welch window
/// https://en.wikipedia.org/wiki/Window_function#Welch_window
/// 
/// Example
/// 
/// ```
/// use dsp::windows;
/// 
/// let win = windows::welch(5, 1, 7);
/// let frame = vec![1.0; 7];
/// let mut output = vec![0.0; 7];
/// win.apply(&frame, &mut output);
/// assert_eq!(output, vec![0.0, 0.0, 0.75, 1.0, 0.75, 0.0, 0.0]);
/// ```
pub fn welch(width: usize, offset: usize, window_length: usize) -> Window {
    let mut samples = vec![0.0; window_length];
    let end = cmp::min(offset + width, window_length);
    let half_width = (width-1) as f32 / 2.0;
    for i in offset..end {
        let n = (i - offset) as f32;
        let y = 1.0 - ((n - half_width) / half_width).powi(2);
        samples[i] = y as f32;
    }
    Window { samples }
}

/// Create the Sine window
/// https://en.wikipedia.org/wiki/Window_function#Sine_window
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::windows;
/// 
/// let win = windows::sine(5, 1, 7);
/// let frame = vec![1.0; 7];
/// let mut output = vec![0.0; 7];
/// win.apply(&frame, &mut output);
/// assert_approx_eq!(output[0], 0.0, 1e-5f32);
/// assert_approx_eq!(output[1], 0.0, 1e-5f32);
/// assert_approx_eq!(output[2], 0.707, 1e-3f32);
/// assert_approx_eq!(output[3], 1.0, 1e-3f32);
/// assert_approx_eq!(output[4], 0.707, 1e-3f32);
/// assert_approx_eq!(output[5], 0.0, 1e-5f32);
/// assert_approx_eq!(output[6], 0.0, 1e-5f32);
/// ```
pub fn sine(width: usize, offset: usize, window_length: usize) -> Window {
    let mut samples = vec![0.0; window_length];
    let end = cmp::min(offset + width, window_length);
    for i in offset..end {
        let n = (i - offset) as f32;
        samples[i] = (PI * n / (width - 1) as f32).sin();
    }
    Window { samples }
}

/// Create the Hann window
/// https://en.wikipedia.org/wiki/Window_function#Hann_and_Hamming_windows
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::windows;
/// 
/// let win = windows::hann(5, 1, 7);
/// let frame = vec![1.0; 7];
/// let mut output = vec![0.0; 7];
/// win.apply(&frame, &mut output);
/// assert_approx_eq!(output[0], 0.0, 1e-5f32);
/// assert_approx_eq!(output[1], 0.0, 1e-5f32);
/// assert_approx_eq!(output[2], 0.5, 1e-3f32);
/// assert_approx_eq!(output[3], 1.0, 1e-3f32);
/// assert_approx_eq!(output[4], 0.5, 1e-3f32);
/// assert_approx_eq!(output[5], 0.0, 1e-5f32);
/// assert_approx_eq!(output[6], 0.0, 1e-5f32);
/// ```
pub fn hann(width: usize, offset: usize, window_length: usize) -> Window {
    let mut samples = vec![0.0; window_length];
    let end = cmp::min(offset + width, window_length);
    for i in offset..end {
        let n = (i - offset) as f32;
        samples[i] = (PI * n / (width - 1) as f32).sin().powi(2);
    }
    Window { samples }
}

/// Compute a hamming window of the given size
/// https://en.wikipedia.org/wiki/Window_function#Hann_and_Hamming_windows
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::windows;
/// 
/// let win = windows::hamming(5, 1, 7);
/// let frame = vec![1.0; 7];
/// let mut output = vec![0.0; 7];
/// win.apply(&frame, &mut output);
/// assert_approx_eq!(output[0], 0.0, 1e-5f32);
/// assert_approx_eq!(output[1], 0.0869, 1e-3f32);
/// assert_approx_eq!(output[2], 0.54347825, 1e-3f32);
/// assert_approx_eq!(output[3], 1.0, 1e-3f32);
/// assert_approx_eq!(output[4], 0.54347825, 1e-3f32);
/// assert_approx_eq!(output[5], 0.0869, 1e-3f32);
/// assert_approx_eq!(output[6], 0.0, 1e-5f32);
/// ```
pub fn hamming(width: usize, offset: usize, window_length: usize) -> Window {
    let a0 = 25.0 / 46.0;
    let mut samples = vec![0.0; window_length];
    let end = cmp::min(offset + width, window_length);
    let size = (width - 1) as f32;
    for i in offset..end {
        let n = (i - offset) as f32;
        let v = a0 - (1.0 - a0) * (2.0 * PI * n / size).cos();
        samples[i] = v;
    }
    Window { samples }
}

/// Compute a Blackman window
/// https://en.wikipedia.org/wiki/Window_function#Blackman_window
/// 
/// Example
/// 
/// ```
/// use assert_approx_eq::assert_approx_eq;
/// use dsp::windows;
/// 
/// let win = windows::blackman(5, 1, 7);
/// let frame = vec![1.0; 7];
/// let mut output = vec![0.0; 7];
/// win.apply(&frame, &mut output);
/// assert_approx_eq!(output[0], 0.0, 1e-5f32);
/// assert_approx_eq!(output[1], 0.00687, 1e-5f32);
/// assert_approx_eq!(output[2], 0.34974, 1e-5f32);
/// assert_approx_eq!(output[3], 1.0, 1e-5f32);
/// assert_approx_eq!(output[4], 0.34974, 1e-5f32);
/// assert_approx_eq!(output[5], 0.00687, 1e-5f32);
/// assert_approx_eq!(output[6], 0.0, 1e-5f32);
/// ```
pub fn blackman(width: usize, offset: usize, window_length: usize) -> Window {
    let a0 = 7938.0 / 18608.0;
    let a1 = 9240.0 / 18608.0;
    let a2 = 1430.0 / 18608.0;
    let mut samples = vec![0.0; window_length];
    let end = cmp::min(offset + width, window_length);
    let size = (width - 1) as f32;
    for i in offset..end {
        let n = (i - offset) as f32;
        let v = a0 - a1 * (2.0 * PI * n / size).cos()
                    + a2 * (4.0 * PI * n / size).cos();
        samples[i] = v;
    }
    Window { samples }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {

    use assert_approx_eq::assert_approx_eq;
    use super::*;

    #[test]
    fn test_window() {
        let win = rectangular(3, 1, 5);
        let frame = vec![1.0; 5];
        let mut output = vec![0.0; 5];
        win.apply(&frame, &mut output);
        assert_eq!(output, vec![0.0, 1.0, 1.0, 1.0, 0.0]);
    }

    #[test]
    fn test_apply() {
        let w = triangular(1000, 0, 1000);
        let frame = vec![1.0; 1000];
        let mut output = vec![0.0; 1000];

        w.apply(&frame, &mut output);
        let area: f32 = output.iter().sum();
        assert_approx_eq!(area / 1000.0, 0.5, 0.2);
    }
}
