//! Audio file sources and Sinks
//! 

use audrey::Reader;


pub struct AudioFileSource {
    reader: Reader<std::io::BufReader<std::fs::File>>,
}

impl AudioFileSource {
    pub fn new(file_path: &str) -> AudioFileSource {
        let reader = audrey::open(file_path).unwrap();
        AudioFileSource {reader}
    }

    pub fn write_buffer(&mut self, buffer: &mut [f32]) {
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