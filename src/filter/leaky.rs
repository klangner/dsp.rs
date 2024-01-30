//! Leaky Integrator implementation
//! Know also as Exponential Smoothing
//! https://en.wikipedia.org/wiki/Exponential_smoothing 
//! 
//! x[t] = alpha * x + (1-alpha)*x[t-1]

#[derive(Clone,Debug)]
pub struct LeakyIntegrator {
    alpha: f32,
    last_value: f32,
}


impl LeakyIntegrator {

    /// Create new LeakyIntegrator filter
    pub fn new(alpha: f32, init_value: f32) -> LeakyIntegrator {
        LeakyIntegrator { alpha, last_value: init_value }
    }

    /// Process next value
    /// 
    /// Example
    /// 
    /// ```
    /// use assert_approx_eq::assert_approx_eq;
    /// use dsp::filter::leaky::LeakyIntegrator;
    /// 
    /// let mut filter = LeakyIntegrator::new(0.1, 16.);
    /// let out = filter.next_value(10.);
    /// 
    /// assert_approx_eq!(out, 15.4, 1e-5f32);
    /// ```
    pub fn next_value(&mut self, v: f32) -> f32 {
        self.last_value = self.alpha*v + (1. - self.alpha)*self.last_value;
        self.last_value
    }


    pub fn process_buffer(&mut self, input_buffer: &[f32], output_buffer: &mut [f32]) {
        let size = std::cmp::min(input_buffer.len(), output_buffer.len());
        for i in 0..size {
            output_buffer[i] = self.next_value(input_buffer[i]);
        }
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_buffer() {
        let mut filter = LeakyIntegrator::new(0.1, 15.);
        let input_buffer: Vec<f32> = vec![10., 20., 100.];
        let mut output_buffer: Vec<f32> = vec![0.; 3];

        let _ = filter.process_buffer(&input_buffer, &mut output_buffer);
        assert_approx_eq!(output_buffer[0], 14.5, 1e-3f32);
        assert_approx_eq!(output_buffer[1], 15.05, 1e-3f32);
        assert_approx_eq!(output_buffer[2], 23.54, 1e-2f32);
    }
}