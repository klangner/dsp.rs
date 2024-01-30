//! Nodes for saving and reading data to/from file
//! 
use std::net::UdpSocket;
use byteorder::{ByteOrder, LittleEndian};
use anyhow::Result;
use crate::node::{SinkNode, SourceNode};


/// Send binary data via the TCP sockek
/// 
/// Example
/// 
/// ```
/// use dsp::node::SinkNode;
/// use dsp::core::network::UdpSink;
/// 
/// let mut node = UdpSink::new(3456, "127.0.0.1:1234");
/// let input_buffer = vec![1.;100];
/// node.read_buffer(&input_buffer);
/// ```
pub struct UdpSink {
    socket: UdpSocket,
    addr: String
}

impl UdpSink {
    pub fn new(port: u32, addr: &str) -> UdpSink {
        let bind_addr = format!("127.0.0.1:{}", port);
        let socket = UdpSocket::bind(bind_addr).expect("Failed to bind UdpSink"); 
        UdpSink {socket, addr: addr.to_owned()}
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
pub struct UdpSource {
    socket: UdpSocket,
    bytes: Vec<u8>,
}

impl UdpSource {
    pub fn new(port: usize, buffer_size: usize) -> UdpSource {
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).expect("Can't bind to udp socket");
        UdpSource {socket, bytes: vec![0; 4*buffer_size]}
    }
}

impl SourceNode<f32> for UdpSource {
    fn write_buffer(&mut self, output_buffer: &mut [f32]) -> Result<()> {
        let _ = self.socket.recv_from(&mut self.bytes).expect("Error reading from socket");
        LittleEndian::read_f32_into(&self.bytes, output_buffer);
        Ok(())
    }
}
