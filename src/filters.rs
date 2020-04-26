/// Basic implementations of common discrete filters
use std::ops::{Add, AddAssign, Mul};
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
        assert_ne!(a[0], Default::default()); // a0 of 0 results in divide by 0

        // Initialize sample histories
        let mut x: ArrayDeque<[f32; 3], Wrapping> = ArrayDeque::new();
        let mut y: ArrayDeque<[f32; 3], Wrapping> = ArrayDeque::new();
        for _ in 0..3 {
            x.push_front(Default::default());
            y.push_front(Default::default());
        }

        // Clone elements from passed in slice
        let mut b_arr: [f32; 3] = Default::default();
        let mut a_arr: [f32; 3] = Default::default();
        b_arr.clone_from_slice(b);
        a_arr.clone_from_slice(a);

        // New filter with x/y initalized to same length as a/b
        BiquadNode {x, y, b: b_arr, a: a_arr, output: Default::default()}
    }

    /// Process one sample of the input signal and returns one sample of the
    /// output signal.
    fn process_one(&mut self, in_samp: f32) -> f32 {

        // Shift in old values
        self.x.pop_back();
        self.x.push_front(in_samp.clone());
        self.y.pop_back();

        // Compute the filter result
        let mut sum = Default::default();
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
