//! Analyze discrete signal in frequency domain using complex numbers

use num_complex::Complex32;
use crate::vector;


/// Calculated frequency of a given component
/// 
pub fn item_freq(data: &[Complex32], sample_rate: usize, i: usize) -> f32 {
    let pos = i % data.len();
    let half_pos = if pos < data.len()/2 { pos } else {data.len() - pos};
    (half_pos * sample_rate) as f32 / data.len() as f32
}

/// Return max frequency of spectrogram data
/// 
pub fn max_freq(data: &[Complex32], sample_rate: usize) -> f32 {
    let buffer: Vec<f32> = data.iter().map(|v| v.norm()).collect();
    let idx = vector::argmax(&buffer);
    if idx < data.len() / 2 {
        item_freq(&data, sample_rate, idx)
    } else {
        item_freq(&data, sample_rate, data.len() - idx)
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
}
