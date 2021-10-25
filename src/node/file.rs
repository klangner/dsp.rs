//! Nodes for saving and reading data to/from file
//! 
use std::fs::File;
use byteorder::WriteBytesExt; 
use byteorder::LittleEndian;
use anyhow::Result;
use crate::runtime::node::SinkNode;


/// Save binary data into a file
/// 
/// Example
/// 
/// ```
/// use dsp::runtime::node::SinkNode;
/// use dsp::node::file::FileSink;
/// 
/// let mut node = FileSink::new("target/file.dat");
/// let input_buffer = vec![1.;100];
/// node.read_buffer(&input_buffer);
/// ```
pub struct FileSink {
    file: Option<File>,
}

impl FileSink {
    pub fn new(file_name: &str) -> FileSink {
        if let Ok(file) = File::create(file_name) {
            FileSink {file: Some(file)}
        } else {
            FileSink{file: None}
        }
    }
}

impl SinkNode<f32> for FileSink {
    fn read_buffer(&mut self, input_buffer: &[f32]) -> Result<()> {
        let mut file = self.file.as_ref().unwrap();
        for v in input_buffer {
            file.write_f32::<LittleEndian>(*v)?;
        }
        let _ = file.sync_all();
        Ok(())
    }
}
