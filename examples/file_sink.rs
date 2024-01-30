use clap::Parser;
use dsp::node::{SourceNode, SinkNode};
use dsp::core::generator::Sine;
use dsp::core::file::FileSink;


const SIGNAL_LENGTH: usize = 512;

#[derive(Parser, Debug)]
struct Args {
    /// Gain to apply to the seify source
    #[clap(short, long, default_value_t = 30)]
    sample_rate: usize,

    /// Center frequency
    #[clap(short, long, default_value_t = 100_000_000.0)]
    freq: f32,
}

fn main() {
    let args = Args::parse();
    let mut generator = Sine::new(args.freq, args.sample_rate);
    let mut file_sink = FileSink::new("target/example.data");
    let mut buffer = vec![0.0; SIGNAL_LENGTH];

    for _ in 1..10 {
        let _ = generator.write_buffer(&mut buffer);
        let _ = file_sink.read_buffer(&buffer);
    }
}