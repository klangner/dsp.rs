#[macro_use]
extern crate clap;

use clap::{Arg, App};
use dsp::runtime::node::{SourceNode, SinkNode};
use dsp::node::generator::Sinusoid;
use dsp::node::file::FileSink;


const SIGNAL_LENGTH: usize = 512;


// Application params
struct Params {
    sample_rate: usize,
    freq: f32
}

/// Parse command line arguments
fn parse_params() -> Params {
    let args = App::new("Save signal to file")
                .arg(Arg::with_name("freq")
                    .short("f")
                    .long("frequency")
                    .help("Frequency in Hz")
                    .takes_value(true))
                .arg(Arg::with_name("sample-rate")
                    .short("s")
                    .long("sample-rate")
                    .help("Number of samples per period")
                    .takes_value(true))
                .get_matches();
    let sample_rate = value_t!(args, "sample-rate", usize).unwrap_or(512);
    let freq = value_t!(args, "freq", f32).unwrap_or(4.0);
    Params { sample_rate: sample_rate,
             freq: freq }
}


fn main() {
    let params = parse_params();
    let mut generator = Sinusoid::new(params.freq, params.sample_rate);
    let mut file_sink = FileSink::new("target/example.data");
    let mut buffer = vec![0.0; SIGNAL_LENGTH];

    for _ in 1..10 {
        let _ = generator.write_buffer(&mut buffer);
        let _ = file_sink.read_buffer(&buffer);
    }
}