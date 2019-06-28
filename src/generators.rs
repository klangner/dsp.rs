//! Signal generators
//! 
//! Signal generators are used to generate different potentially infinite signals
//! Each generator has state and can fill buffer with data.
//! 

use std::f32;
// use std::f32::consts::PI;


pub trait SignalGen {
    fn next(&mut self, buffer: &mut Vec<f32>) -> usize;
}

/// Impulse signal
/// x[n] = 1 if n == impulse_pos
/// x[n] = 0 if n != impulse_pos
pub struct ImpulseGen {
    current_sample: i64, 
    impulse_pos: i64,
}

impl ImpulseGen {
    /// Create new Impulse generator with impulse moved to specyfic position
    /// 
    /// Example
    /// 
    /// ```
    /// use dsp::generators::{SignalGen, ImpulseGen};
    /// 
    /// let mut gen = ImpulseGen::new(3);
    /// let mut buffer = vec![0.0; 5];
    /// gen.next(&mut buffer);
    /// assert_eq!(buffer, vec![0.0, 0.0, 0.0, 1.0, 0.0]);
    /// ```
    pub fn new(impulse_pos: i64) -> ImpulseGen {
        ImpulseGen { current_sample : 0, impulse_pos }
    }
}

impl SignalGen for ImpulseGen {

    fn next(&mut self, buffer: &mut Vec<f32>) -> usize {
        for sample in buffer.iter_mut() {                        
            *sample = if self.current_sample == self.impulse_pos { 1.0 } else { 0.0 };
            self.current_sample += 1;
        }
        buffer.len()
    }
}

/// Step signal
/// x[n] = 1 if n > step_pos
/// x[n] = 0 if n < step_pos
pub struct StepGen {
    current_sample: i64, 
    step_pos: i64,
}

impl StepGen {
    /// Create new Step generator with step moved to specyfic position
    /// 
    /// Example
    /// 
    /// ```
    /// use dsp::generators::{SignalGen, StepGen};
    /// 
    /// let mut gen = StepGen::new(3);
    /// let mut buffer = vec![0.0; 5];
    /// gen.next(&mut buffer);
    /// assert_eq!(buffer, vec![0.0, 0.0, 0.0, 1.0, 1.0]);
    /// ```
    pub fn new(step_pos: i64) -> StepGen {
        StepGen { current_sample : 0, step_pos }
    }
}

impl SignalGen for StepGen {

    fn next(&mut self, buffer: &mut Vec<f32>) -> usize {
        for sample in buffer.iter_mut() {                        
            *sample = if self.current_sample >= self.step_pos { 1.0 } else { 0.0 };
            self.current_sample += 1;
        }
        buffer.len()
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impulse() {
        let mut gen = ImpulseGen::new(3);
        let mut buffer = vec![0.0; 500];
        gen.next(&mut buffer);
        assert_eq!(buffer.iter().sum::<f32>(), 1.0);
    }

}
