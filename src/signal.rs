/// Signal is function f: R -> C
/// Where:
///  R - Set of Real Numbers (Here defined as f32)
///  C - Set of Complex Numbers (Here defines as Complex32)

use num_complex::{Complex, Complex32};


pub struct Signal{
    gen: Box<Fn(f32) -> Complex32>
}

impl Signal {
    pub fn at(&self, i: f32) -> Complex32 {
        (self.gen)(i)
    }
}


/// Impulse signal
/// x[n] = 1 if n == 0
/// x[n] = 0 if n > 0
pub fn impulse() -> Signal {
    Signal { gen: Box::new(|i| if i == 0. {Complex::new(1., 0.)} else {Complex::new(0., 0.)}) }
}

/// Step signal
/// x[n] = 1 if n >= 0
/// x[n] = 0 if n < 0
pub fn step() -> Signal {
    Signal { gen: Box::new(|i| if i >= 0. {Complex::new(1., 0.)} else {Complex::new(0., 0.)}) }
}

///// Base function for Discrete Fourier Transformation
//pub fn dft_base(n: f32) -> Complex32 {
//    if n >= 0. {
//        Complex::new(1., 0.)
//    } else {
//        Complex::new(0., 0.)
//    }
//}

/// Sample given signal
pub fn sample(signal: &Signal, start: f32, end: f32, step: f32) -> Vec<Complex32> {
    let size = ((end-start)/step) as usize;
    let mut v: Vec<Complex32> = Vec::with_capacity(size);
    let mut i = start;
    while i <= end {
        v.push(signal.at(i));
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
        let signal = impulse();
        assert!(signal.at(-4.0) == Complex::new(0., 0.));
        assert!(signal.at(0.) == Complex::new(1., 0.));
        assert!(signal.at(42.) == Complex::new(0., 0.));
    }

    #[test]
    fn test_sample() {
        let signal = impulse();
        let xs = sample(&signal, -1.0, 1.0, 1.);
        assert!(xs == vec![Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)]);
    }
}