/// Helper functions for vector operations based on block of data
/// All operations here should be alloc free 
use std::f32;


/// Multiply vectors element wise
pub fn multiply(xs: &Vec<f32>, ys: &Vec<f32>, output: &mut Vec<f32>) {
    assert_eq!(xs.len(), output.len());
    assert_eq!(ys.len(), output.len());
    for i in 0..output.len() {
        output[i] = xs[i] * ys[i];
    }
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