//! Nodes for saving and reading data to/from file
//! 
use std::fs::File;
use byteorder::{ReadBytesExt, WriteBytesExt}; 
use byteorder::LittleEndian;


/// Save binary data into a file
/// 
/// Example
/// 
/// ```
/// use dsp::core::file::FileSink;
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

    pub fn read_buffer(&mut self, input_buffer: &[f32]) {
        let mut file = self.file.as_ref().unwrap();
        for v in input_buffer {
            file.write_f32::<LittleEndian>(*v).expect("Can't reead from file");
        }
        let _ = file.sync_all();
    }
}


/// Load binary data from a file
/// 
pub struct FileSource {
    file: File,
}

impl FileSource {
    pub fn new(file_name: &str) -> FileSource {
        let file = File::open(file_name).expect("Can't open file");
        FileSource {file}
    }

    pub fn write_buffer(&mut self, output_buffer: &mut [f32]) {
        for i in 0..output_buffer.len() {
            if let Ok(v) = self.file.read_f32::<LittleEndian>() {
                output_buffer[i] = v;
            } else {
                break
            }
        }
    }
}
