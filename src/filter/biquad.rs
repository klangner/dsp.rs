/// Basic implementations of common discrete filters
use arraydeque::{ArrayDeque, Wrapping};
use itertools::izip;
use anyhow::Result;

use crate::node::ProcessNode;


/// A biquad filter (IIR)
#[derive(Clone,Debug)]
pub struct BiquadFilter {
    x: ArrayDeque<f32, 3, Wrapping>,
    y: ArrayDeque<f32, 3, Wrapping>,
    b: [f32; 3],
    a: [f32; 3]
}


impl BiquadFilter {

    /// Returns a new biquad IIR filter. Failure if a/b not correct lengths
    pub fn new(b: &[f32], a: &[f32]) -> BiquadFilter {

        // Sanity check
        assert_eq!(b.len(), 3);
        assert_eq!(a.len(), 3);
        assert_ne!(a[0], 0.0); // a0 of 0 results in divide by 0

        // Initialize sample histories
        let mut x: ArrayDeque<f32, 3, Wrapping> = ArrayDeque::new();
        let mut y: ArrayDeque<f32, 3, Wrapping> = ArrayDeque::new();
        for _ in 0..3 {
            x.push_front(0.0);
            y.push_front(0.0);
        }

        // Clone the b coefficients from passed in slice
        let mut b_arr: [f32; 3] = [0.0; 3];
        b_arr.clone_from_slice(b);

        // Clone the a coefficients, inverting a[1..] by
        // the definition of an IIR filter
        let mut neg_a_arr: [f32; 3] = [0.0; 3];
        neg_a_arr[0] = a[0];
        for i in 1..3 {
            neg_a_arr[i] = -a[i];
        }

        // New filter with x/y initalized to same length as a/b
        BiquadFilter {
            x, y,
            b: b_arr, a: neg_a_arr
        }
    }

    /// Process one sample of the input signal and returns one sample of the
    /// output signal.
    fn process_one(&mut self, in_samp: f32) -> f32 {

        // Shift in old values
        self.x.pop_back();
        self.x.push_front(in_samp.clone());
        self.y.pop_back();

        // Compute the filter result
        let mut sum = 0.0;
        for (xi, bi) in izip!(self.x.iter(), self.b.iter()) {
            sum += *xi * *bi;
        }
        for (yi, ai) in izip!(self.y.iter(), self.a[1..].iter()) {
            sum += *yi * *ai;
        }
        sum /= self.a[0];

        // Update y and return the result
        self.y.push_front(sum);
        sum
    }

}

impl ProcessNode<f32, f32> for BiquadFilter {

    fn process_buffer(&mut self, input_buffer: &[f32], output_buffer: &mut [f32]) -> Result<()> {
        let size = std::cmp::min(input_buffer.len(), output_buffer.len());
        (0..size).for_each(|i| output_buffer[i] = self.process_one(input_buffer[i]));
        Ok(())
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
    fn test_biquad_bilinear_rc() {

        // Test our biquad by using the bilinear transform to create
        // a digital filter with similar response to a basic analog RC
        // filter with constants T=0.1s RC=1.
        // Generate and compare their step responses over 5 seconds.
        let rc = 1.0;
        let t_samp = 0.1;

        // Analog step response evaluated every T seconds.
        let mut analog_response = Vec::new();
        for i in 0..50 {
            let t = (i as f32) * t_samp;
            analog_response.push(1.0 - f32::exp(-t / rc));
        }

        // Compute and run the equivalent digital filter.
        // Bilinear transform of RC filter:
        // https://en.wikipedia.org/wiki/Bilinear_transform#Example
        let b = [1.0, 1.0, 0.0];
        let a = [1.0+(2.0*rc/t_samp), 1.0-(2.0*rc/t_samp), 0.0];
        let mut biquad_rc = BiquadFilter::new(&b, &a);
        let dig_unit_step = vec![1.0; 50];
        let mut digital_response = vec![0.0; 50];
        let _ = biquad_rc.process_buffer(&dig_unit_step, &mut digital_response);
        
        // Compare the filter unit step responses. Since there
        // is some difference between the initial state of the
        // filters, use a less aggressive 5% threshold
        for i in 0..50 {
            assert_approx_eq!(analog_response[i], digital_response[i], 0.05);
        }
    }
}
