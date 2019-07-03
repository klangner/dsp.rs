//! Analyze discrete signal in frequency domain using xomplex numbers

use crate::ComplexBuffer;
use crate::vectors;


/// Calculated frequency of a given component
pub fn item_freq(i: usize, spectrum : &ComplexBuffer, sample_rate: usize) -> f32 {
    (i * sample_rate) as f32 / (spectrum.len() as f32)
}

/// Return max frequency
pub fn max_freq(spectrum : &ComplexBuffer, sample_rate: usize) -> f32 {
    let idx = vectors::argmax(&spectrum);
    if idx < spectrum.len() / 2 {
        item_freq(idx, &spectrum, sample_rate)
    } else {
        item_freq(spectrum.len() - idx, &spectrum, sample_rate)
    }
}
