/// Signal is function f: R -> C
/// Where:
///  R - Set of Real Numbers (Here defined as f64)
///  C - Set of Complex Numbers (Here defines as Complex64)

use std::f64;
use num_complex::{Complex, Complex64};


pub struct Signal{
    gen: Box<Fn(f64) -> Complex64>
}

impl Signal {
    pub fn at(&self, i: f64) -> Complex64 {
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

/// Complex sinusoidal signal
pub fn complex(freq: f64, offset: f64) -> Signal {
    let w = 2.0*f64::consts::PI/freq;
    Signal { gen: Box::new(move |i| Complex::new(0., w*i + offset).exp()) }
}


/// Real value sinusoidal signal
pub fn sinusoid(freq: f64, offset: f64) -> Signal {
    let w = 2.0*f64::consts::PI/freq;
    Signal { gen: Box::new(move |i| Complex::new(f64::sin(w*i + offset), 0.).exp()) }
}


/// Sample given signal
pub fn sample(signal: &Signal, start: f64, end: f64, step: f64) -> Vec<Complex64> {
    let size = ((end-start)/step) as usize;
    let mut v: Vec<Complex64> = Vec::with_capacity(size);
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