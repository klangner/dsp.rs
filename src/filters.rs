// TODO:
// - no_std by moving all a/b to slices
// - benchmark VecDeque vs C method vs custom ringbuffer

use std::ops::{Add, AddAssign, Mul};

use arraydeque::{ArrayDeque, Wrapping};
use itertools::{izip};


#[derive(Clone,Debug)]
pub struct IIRNaive<T> {
    x: ArrayDeque<[T; 3], Wrapping>,
    y: ArrayDeque<[T; 3], Wrapping>,
    b: [T; 3],
    a: [T; 3],
}


impl<T: Default + Copy + Add + AddAssign + Mul<Output=T>> IIRNaive<T> {

    // Returns a new IIR filter. Returns Fail if a/b not correct lengths
    pub fn new(b_in: &[T], a_in: &[T]) -> Result<IIRNaive<T>, &'static str> {

        // Initialize sample histories
        let mut x: ArrayDeque<[T; 3], Wrapping> = ArrayDeque::new();
        let mut y: ArrayDeque<[T; 3], Wrapping> = ArrayDeque::new();
        for _ in 0..3 {
            x.push_front(Default::default());
            y.push_front(Default::default());
        }

        // Sanity check
        if x.len() != b_in.len() || y.len() != a_in.len() {
            return Err("Input slices contained an incorrect number of elements")
        }

        // Clone elements from passed in slice
        let mut b: [T; 3] = Default::default();
        let mut a: [T; 3] = Default::default();
        b.clone_from_slice(b_in);
        a.clone_from_slice(a_in);

        // New filter with x/y initalized to same length as a/b
        Ok(IIRNaive {x, y, b, a})
    }

    pub fn consume(&mut self, in_samp: &T) -> T {

        // Shift in old values
        self.x.pop_back();
        self.x.push_front(in_samp.clone());
        self.y.pop_back();

        // Run the filter
        let mut sum = Default::default();
        for (xi, bi) in izip!(self.x.iter(), self.b.iter()) {
            sum += *xi * *bi;
        }
        for (yi, ai) in izip!(self.y.iter(), self.a[1..].iter()) {
            sum += *yi * *ai;
        }

        // Update y and return the result
        self.y.push_front(sum);
        sum
    }

    pub fn consume_slice(&mut self, in_slice: &[T], out_slice: &mut [T]) {

        for (in_samp, out_samp) in izip!(in_slice.iter(), out_slice.iter_mut()) {
            *out_samp = self.consume(in_samp);
        };
    }
}
