//! #  Signal definition

/// Signal consists of:
/// * data
/// * sample_rate which is number of samples per second (Sampling frequency)
pub struct Signal {
    pub data: Vec<f32>,
    pub sample_rate: usize
}

impl Signal {

    /// Create empty signal
    pub fn empty(sample_rate: usize) -> Signal {
        Signal::new(vec![], sample_rate)
    }

    /// Create new signal from provided data
    /// 
    /// Example
    /// 
    /// ```
    /// use dsp::signal::Signal;
    /// 
    /// let signal = Signal::new(vec![1.0, 2.0, 3.0], 2);
    /// assert_eq!(signal.len(), 3);
    /// ```
    pub fn new(data: Vec<f32>, sample_rate: usize) -> Signal {
        Signal { data, sample_rate }
    }

    /// Length of the signal 
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Rescale signal
    /// 
    /// Example
    /// 
    /// ```
    /// use dsp::signal::Signal;
    /// 
    /// let signal = Signal::new(vec![1.0, 2.0, 3.0], 2);
    /// let rescaled = signal.rescale(0.5);
    /// assert_eq!(rescaled.data, vec![0.5, 1.0, 1.5]);
    /// ```
    pub fn rescale(&self, amount: f32) -> Signal {
        let data = self.data.iter().map(|v| amount*v).collect();
        Signal::new(data, self.sample_rate)
    }

}
