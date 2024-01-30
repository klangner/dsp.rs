use dsp::core::generator::Sine;
use dsp::core::network::UdpSink;


const SAMPLE_RATE: usize = 32_000;
const BUFFER_SIZE: usize = 1024;
const FREQ: f32 = 440.;



fn main() {
    let mut generator = Sine::new(FREQ, SAMPLE_RATE);
    let mut tcp_sink = UdpSink::new(1212, "127.0.0.1:1234");
    let mut buffer = vec![0.0; BUFFER_SIZE];

    loop {
        let _ = generator.write_buffer(&mut buffer);
        let _ = tcp_sink.read_buffer(&buffer);
    }
}