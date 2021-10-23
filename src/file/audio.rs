//! Audio file sources and Sinks
//! 

use crate::node::{SourceNode};
use audrey::Reader;


pub struct AudioSource {
    reader: Reader<std::io::BufReader<std::fs::File>>,
}

impl AudioSource {
    pub fn new(file_path: &str) -> AudioSource {
        let reader = audrey::open(file_path).unwrap();
        AudioSource {reader}
    }
}

impl SourceNode<f32> for AudioSource {
    fn write_buffer(&mut self, buffer: &mut [f32]) {
        let mut samples = self.reader.samples(); 
        for i in 0..buffer.len() {
            if let Some(v) = samples.next() {
                buffer[i] = v.unwrap();
            } else {
                break;
            }
        }
    }
}