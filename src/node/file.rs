//! Nodes for saving and reading data to/from file
//! 
use std::fs::File;
use byteorder::{ReadBytesExt, WriteBytesExt}; 
use byteorder::LittleEndian;
use anyhow::Result;
use crate::runtime::node::{SinkNode, SourceNode};


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


/// Load binary data from file
/// 
/// Example
/// 
/// ```
/// use dsp::runtime::node::SourceNode;
/// use dsp::node::file::FileSource;
/// 
/// let mut node = FileSource::new("target/file.dat");
/// let mut input_buffer = vec![1.;100];
/// node.write_buffer(&mut input_buffer);
/// ```
pub struct FileSource {
    file: Option<File>,
}

impl FileSource {
    pub fn new(file_name: &str) -> FileSource {
        if let Ok(file) = File::open(file_name) {
            FileSource {file: Some(file)}
        } else {
            FileSource{file: None}
        }
    }
}

impl SourceNode<f32> for FileSource {
    fn write_buffer(&mut self, output_buffer: &mut [f32]) -> Result<()> {
        let mut file = self.file.as_ref().unwrap();
        for i in 0..output_buffer.len() {
            if let Ok(v) = file.read_f32::<LittleEndian>() {
                output_buffer[i] = v;
            } else {
                break
            }
        }
        Ok(())
    }
}
