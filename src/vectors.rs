/// Helper functions for vector operations based on block of real data
/// All operations here should be alloc free 
use std::ops::Mul;
use std::f32;
use crate::num_complex::Complex32;


/// Multiply vectors element wise
pub fn multiply<T>(xs: &Vec<T>, ys: &Vec<T>, output: &mut Vec<T>) 
where
    T: Mul<Output = T> + Copy
{
    assert_eq!(xs.len(), output.len());
    assert_eq!(ys.len(), output.len());
    for i in 0..output.len() {
        output[i] = xs[i] * ys[i];
    }
}

/// Calculate arg max for complex numbers
pub fn argmax(xs: &Vec<Complex32>) -> usize {
    let mut max_value = f32::MIN;
    let mut arg_max = 0;
    for i in 0..xs.len() {
        let v = xs[i].norm();
        if max_value < v {
            max_value = v;
            arg_max = i;
        }
    }
    arg_max
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let v = vec![3.0; 5];
        let u = vec![2.0; 5];
        let mut output = vec![0.0; 5];
        multiply(&v, &u, &mut output);
        assert_eq!(output, vec![6.0; 5]);
    }
}