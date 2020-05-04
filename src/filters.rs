/// Basic implementations of common discrete filters
use arraydeque::{ArrayDeque, Wrapping};
use itertools::{izip};
use crate::{RealBuffer, ProcessingNode};


/// A biquad filter (IIR)
#[derive(Clone,Debug)]
pub struct BiquadNode {
    x: ArrayDeque<[f32; 3], Wrapping>,
    y: ArrayDeque<[f32; 3], Wrapping>,
    b: [f32; 3],
    a: [f32; 3],
    output: RealBuffer,
}


impl BiquadNode {

    /// Returns a new biquad IIR filter. Failure if a/b not correct lengths
    pub fn new(b: &[f32], a: &[f32]) -> BiquadNode {

        // Sanity check
        assert_eq!(b.len(), 3);
        assert_eq!(a.len(), 3);
        assert_ne!(a[0], 0.0); // a0 of 0 results in divide by 0

        // Initialize sample histories
        let mut x: ArrayDeque<[f32; 3], Wrapping> = ArrayDeque::new();
        let mut y: ArrayDeque<[f32; 3], Wrapping> = ArrayDeque::new();
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
        BiquadNode {
            x, y,
            b: b_arr, a: neg_a_arr,
            output: Default::default()
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

impl ProcessingNode for BiquadNode {
    type InBuffer = RealBuffer;
    type OutBuffer = RealBuffer;

    /// Processes in_slice as a slice of samples as inputs to the filter,
    /// writing results to out_slice.
    fn process(&mut self, input: &RealBuffer) -> &RealBuffer {

        // Re-allocate our output buffer to match our input size
        self.output = Vec::with_capacity(input.len());

        // Run process_one() for all elements in the input, updating the output
        for in_samp in input.iter() {
            let out_samp = self.process_one(*in_samp);
            self.output.push(out_samp);
        }

        // Return a ref to the now-valid output buffer
        &self.output
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
        println!("a: {:#?}", a);
        let mut biquad_rc = BiquadNode::new(&b, &a);
        let dig_unit_step = vec![1.0; 50];
        let digital_response = biquad_rc.process(&dig_unit_step);
        
        // Compare the filter unit step responses. Since there
        // is some difference between the initial state of the
        // filters, use a less aggressive 5% threshold
        for i in 0..50 {
            assert_approx_eq!(analog_response[i], digital_response[i], 0.05);
        }
    }
}
