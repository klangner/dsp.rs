//! Audio file sources and Sinks
//! 

use anyhow::Result;
use crate::runtime::node::{SourceNode};
use audrey::Reader;


pub struct AudioFileSource {
    reader: Reader<std::io::BufReader<std::fs::File>>,
}

impl AudioFileSource {
    pub fn new(file_path: &str) -> AudioFileSource {
        let reader = audrey::open(file_path).unwrap();
        AudioFileSource {reader}
    }
}

impl SourceNode<f32> for AudioFileSource {
    fn write_buffer(&mut self, buffer: &mut [f32]) -> Result<()> {
        let mut samples = self.reader.samples(); 
        for i in 0..buffer.len() {
            if let Some(v) = samples.next() {
                buffer[i] = v.unwrap();
            } else {
                break;
            }
        }
        Ok(())
    }
}