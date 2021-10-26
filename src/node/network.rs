//! Nodes for saving and reading data to/from file
//! 
use std::net::{UdpSocket};
use byteorder::{ByteOrder, LittleEndian};
use anyhow::Result;
use crate::runtime::node::{SinkNode, SourceNode};


/// Send binary data via the TCP sockek
/// 
/// Example
/// 
/// ```
/// use dsp::runtime::node::SinkNode;
/// use dsp::node::tcp::UdpSink;
/// 
/// let mut node = UdpSink::new(3456);
/// let input_buffer = vec![1.;100];
/// node.read_buffer(&input_buffer);
/// ```
pub struct UdpSink {
    socket: UdpSocket,
    addr: String
}

impl UdpSink {
    pub fn new(port: u32, addr: String) -> UdpSink {
        let bind_addr = format!("127.0.0.1:{}", port);
        let socket = UdpSocket::bind(bind_addr).expect("Failed to bind UdpSink"); 
        UdpSink {socket, addr}
    }
}

impl SinkNode<f32> for UdpSink {
    fn read_buffer(&mut self, input_buffer: &[f32]) -> Result<()> {
        let mut bytes: Vec<u8> = vec![0; 4 * input_buffer.len()];
        LittleEndian::write_f32_into(&input_buffer, &mut bytes);
        self.socket.send_to(&bytes, &self.addr).unwrap();
        Ok(())
    }
}


/// Load binary data from file
/// 
/// Example
/// 
/// ```
/// //use dsp::runtime::node::SourceNode;
/// //use dsp::node::file::TcpSource;
/// 
/// //let mut node = tcpSource::new("target/file.dat");
/// //let mut input_buffer = vec![1.;100];
/// //node.write_buffer(&mut input_buffer);
/// ```
pub struct TcpSource {
}
/*
impl TcpSource {
    pub fn new(file_name: &str) -> TcpSource {
        if let Ok(file) = File::open(file_name) {
            TcpSource {file: Some(file)}
        } else {
            TcpSource{file: None}
        }
    }
}

impl SourceNode<f32> for TcpSource {
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
*/