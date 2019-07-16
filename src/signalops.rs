

trait SignalOp {
    fn process(&self, sample: f32) -> f32;
}

/// Change signal amplitude
pub struct AmplitudeOp {
    volume: f32,
}

impl AmplitudeOp {
    pub fn new(volume: f32) -> AmplitudeOp {
        AmplitudeOp { volume }
    }
}

impl SignalOp for AmplitudeOp {
    fn process(&self, sample: f32) -> f32 {
        self.volume * sample
    }
}

/// Add 2 signals
pub struct AddOp {

}

// impl AddOp {
//     pub fn new() -> AmplitudeOp {
//         AddOp {}
//     }
// }

// impl SignalOp for AddOp {

// }