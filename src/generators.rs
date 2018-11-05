//! Signal generators

use std::f64;
use std::f64::consts::PI;
use rand;
use rand::distributions::{Normal, IndependentSample};
use num_complex::{Complex, Complex64};
use signals::{Signal};


pub struct SignalGen {
    gen: Box<Fn(f64) -> Complex64>
}

impl SignalGen {
    /// Generate signal at given points
    pub fn generate(&self, points: Vec<f64>) -> Signal {
        let data = points.iter().map(|&i| (self.gen)(i)).collect();
        Signal::new(data)
    }
}


/// Impulse signal
/// x[n] = 1 if n == 0
/// x[n] = 0 if n > 0
pub fn impulse() -> SignalGen {
    SignalGen { gen: Box::new(|i| if i == 0. {Complex::new(1., 0.)} else {Complex::new(0., 0.)}) }
}

/// Step signal
/// x[n] = 1 if n >= 0
/// x[n] = 0 if n < 0
pub fn step() -> SignalGen {
    SignalGen { gen: Box::new(|i| if i >= 0. {Complex::new(1., 0.)} else {Complex::new(0., 0.)}) }
}

/// Complex sinusoidal signal
pub fn complex(freq: f64, offset: f64) -> SignalGen {
    let w = 2.0*PI*freq;
    SignalGen { gen: Box::new(move |i| Complex::new(0., w*(i + offset/2.)).exp()) }
}


/// Real value sine signal
pub fn sine(freq: f64, offset: f64) -> SignalGen {
    let w = 2.0*PI*freq;
    SignalGen { gen: Box::new(move |i| Complex::new(f64::sin(w*(i + offset/2.)), 0.)) }
}


/// Real value cosine signal
pub fn cosine(freq: f64, offset: f64) -> SignalGen {
    let w = 2.0*PI*freq;
    SignalGen { gen: Box::new(move |i| Complex::new(f64::cos(w*(i + offset/2.)), 0.)) }
}


/// Real value periodic triangle signal (with period of 1 second).
pub fn triangle(freq: f64) -> SignalGen {
    let w = 2.0*freq;
    SignalGen { gen: Box::new(move |i| Complex::new((w*(i+0.5)) % 2. - 1., 0.)) }
}

/// Real value periodic square signal (with period of 1 second).
pub fn square(freq: f64) -> SignalGen {
    let w = freq;
    SignalGen { gen: Box::new(move |i| {
        let a = w*i % 1.;
        let b = if a < -0.5 || (a > 0.0 && a < 0.5) {1.0} else  {-1.0};
        Complex::new(b, 0.)
    })}
}


/// A chirp is a signal in which frequency increases with time.
pub fn chirp(start_freq: f64, end_freq: f64, time: f64) -> SignalGen {
    let slope = (end_freq - start_freq)/time;
    SignalGen { gen: Box::new(move |i| {
        if i < 0. || i > time {
            Complex::new(0., 0.)
        } else {
            let f = slope*i + start_freq;
            let w = 2.0*PI*f*i;
            Complex::new(0., w).exp()
        }
    })}
}

/// A real noise (without imaginary part)
pub fn noise(std: f64) -> SignalGen {
    let normal = Normal::new(0.0, std);
    SignalGen { gen: Box::new(move |_| {
        let mut rng = rand::thread_rng();
        Complex::new(normal.ind_sample(&mut rng), 0.0)
    })}
}

/// A arbitrary signals generator
pub fn arbitrary(f: Box<Fn(f64) -> Complex64>) -> SignalGen {
    SignalGen{f}
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
        let signal = impulse().generate(vec![-4.0, 0.0, 42.0]);
        assert_eq!(signal.get(0), Complex::new(0., 0.));
        assert_eq!(signal.get(1), Complex::new(1., 0.));
        assert_eq!(signal.get(2), Complex::new(0., 0.));
    }

    #[test]
    fn test_step() {
        let signal = step().generate(vec![-4.0, 0.0, 42.0]);
        assert_eq!(signal.get(0), Complex::new(0., 0.));
        assert_eq!(signal.get(1), Complex::new(1., 0.));
        assert_eq!(signal.get(2), Complex::new(1., 0.));
    }

    #[test]
    fn test_generate() {
        let signal = impulse().generate(vec![-4.0, 0.0, 42.0]);
        assert_eq!(signal.get(0), Complex::new(0., 0.));
        assert_eq!(signal.get(1), Complex::new(1., 0.));
        assert_eq!(signal.get(2), Complex::new(0., 0.));
    }
}
