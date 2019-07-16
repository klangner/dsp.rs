#[macro_use]
extern crate clap;

use gnuplot::{Figure, Color};
use clap::{Arg, App};
use dsp::SourceNode;
use dsp::generators::*;


// Application params
struct Params {
    gen_name: String,
    sample_freq: f32,
    freq: f32
}

/// Parse command line arguments
fn parse_params() -> Params {
    let args = App::new("Plot signal")
                .arg(Arg::with_name("gen")
                    .short("g")
                    .long("generator-name")
                    .help("Select generator type")
                    .takes_value(true))
                .arg(Arg::with_name("freq")
                    .short("f")
                    .long("frequency")
                    .help("Frequency in Hz")
                    .takes_value(true))
                .arg(Arg::with_name("sample-rate")
                    .short("s")
                    .long("sample-rate")
                    .help("Sampling frequency")
                    .takes_value(true))
                .get_matches();
    let gen_name = args.value_of("gen").unwrap_or("sine"); 
    let sample_rate = value_t!(args, "sample-rate", f32).unwrap_or(512.0);
    let freq = value_t!(args, "freq", f32).unwrap_or(4.0);
    Params { gen_name: gen_name.to_string(),
             sample_freq: sample_rate,
             freq: freq }
}

/// Create Signal generator based on given params
fn create_generator(params: &Params) -> Box<SignalGen + 'static> {
    match params.gen_name.as_ref() {
        "triangle"  => Box::new(TriangleGen::new(params.freq, params.sample_freq)),
        "square"    => Box::new(SquareGen::new(params.freq, params.sample_freq)),
        "noise"     => Box::new(NoiseGen::new(0.4)),
        "chirp"     => Box::new(ChirpGen::new(1.0, 50.0, 1.0, params.sample_freq)),
        _           => Box::new(SineGen::new(params.freq, params.sample_freq)),
    }
}

fn main() {
    let params = parse_params();
    let gen = create_generator(&params);
    let mut gen_node = GenNode::new(gen, params.sample_freq as usize);
    let buffer = gen_node.next_frame();

    // Plot signal
    let idx: Vec<usize> = (0..params.sample_freq as usize).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, buffer, &[Color("red")]);
    fg.show();
}