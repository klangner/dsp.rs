//! Helper functions for discrete signal in time domain 


/// Calculated signal power as 
pub fn power(buffer: &[f32]) -> f32 {
    let s:f32 = buffer.iter()
        .map(|i| f32::powf(*i, 2.))
        .sum();
    s / buffer.len() as f32
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use crate::core::generator::Sine;
    use super::*;


    #[test]
    fn test_power() {
        let mut generator = Sine::new(1_000., 8_192);
        let mut buffer = vec![0.0; 8_192];
        let _ = generator.write_buffer(&mut buffer);
        let power = power(&buffer);
        assert_approx_eq!(power, 0.5, 1e-5f32);
    }
}
