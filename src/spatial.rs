/// Spatial domain processing (e.g. Time)

use ndarray::{Array, Ix1};


/// One dimensional signal
pub type Vector = Array<f32, Ix1>;

/// Create new signal from vector data.
/// This vector will be owned by Signal.
pub fn signal(v: Vec<f32>) -> Vector {
    Array::from_vec(v)
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_vector() {
        let v: Vector = signal(vec![1., 2., 3., 4.]);
        assert!(v.ndim() == 1);
        assert!(v.len() == 4);
    }
}