/// Signal is function f: R -> C
/// Where:
///  R - Set of Real Numbers (Here defined as f32)
///  C - Set of Complex Numbers (Here defines as Complex32)

use num_complex::{Complex, Complex32};


/// Impulse signal
/// x[n] = 1 if n == 0
/// x[n] = 0 if n > 0
pub fn impulse(n: f32) -> Complex32 {
    if n == 0. {
        Complex::new(1., 0.)
    } else {
        Complex::new(0., 0.)
    }
}

/// Step signal
/// x[n] = 1 if n >= 0
/// x[n] = 0 if n < 0
pub fn step(n: f32) -> Complex32 {
    if n >= 0. {
        Complex::new(1., 0.)
    } else {
        Complex::new(0., 0.)
    }
}

/// Sample given signal
pub fn sample(signal: &Fn(f32) -> Complex32, start: f32, end: f32, step: f32) -> Vec<Complex32> {
    let size = ((end-start)/step) as usize;
    let mut v: Vec<Complex32> = Vec::with_capacity(size);
    let mut i = start;
    while i <= end {
        v.push(signal(i));
        i += step;
    }
    v
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use num_complex::{Complex};
    use super::*;

    #[test]
    fn test_impulse() {
        assert!(impulse(-4.0) == Complex::new(0., 0.));
        assert!(impulse(0.) == Complex::new(1., 0.));
        assert!(impulse(42.) == Complex::new(0., 0.));
    }
}