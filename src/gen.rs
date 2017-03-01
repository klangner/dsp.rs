/// Signal generators

use num_complex::{Complex, Complex32};
use signals::{Signal, signal};


/// Signal generator
/// Generates size-samples using provided as closure function
pub fn signal_generator(size: usize, g: &Fn(usize) -> Complex32) -> Signal {
    let mut vs: Vec<Complex32> = Vec::with_capacity(size);
    for n in 0..size {
        vs.push(g(n));
    }
    signal(vs)
}

/// Impulse signal
/// x[n] = 1 if n == 0
/// x[n] = 0 if n > 0
pub fn impulse(size: usize) -> Signal {
    let g = |n| if n == 0 {Complex::new(1., 0.)} else {Complex::new(0., 0.)};
    signal_generator(size, &g)
}


/// Step signal
/// x[n] = 1 if n >= 0
/// x[n] = 0 if n < 0
pub fn step(size: usize) -> Signal {
    let g = |_| Complex::new(1., 0.);
    signal_generator(size, &g)
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
        let v = impulse(4);
        assert!(v.ndim() == 1);
        assert!(v.len() == 4);
        assert!(v == signal(vec![Complex::new(1., 0.),
                                 Complex::new(0., 0.),
                                 Complex::new(0., 0.),
                                 Complex::new(0., 0.)]));
    }
}