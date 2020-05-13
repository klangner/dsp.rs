#[macro_use]
extern crate clap;

use gnuplot::{Figure, Color};
use clap::{Arg, App};
use dsp::Signal;
use dsp::generators::*;


const SIGNAL_LENGTH: usize = 1_000;


// Application params
struct Params {
    gen_name: String,
    sample_rate: usize,
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
                    .help("Number of samples per period")
                    .takes_value(true))
                .get_matches();
    let gen_name = args.value_of("gen").unwrap_or("sine"); 
    let sample_rate = value_t!(args, "sample-rate", usize).unwrap_or(512);
    let freq = value_t!(args, "freq", f32).unwrap_or(4.0);
    Params { gen_name: gen_name.to_string(),
             sample_rate: sample_rate,
             freq: freq }
}

/// Create signal
fn create_signal(params: &Params) -> Signal {
    match params.gen_name.as_ref() {
        "triangle"  => traingle(SIGNAL_LENGTH, params.freq, params.sample_rate),
        "square"    => square(SIGNAL_LENGTH, params.freq, params.sample_rate),
        "noise"     => noise(SIGNAL_LENGTH, 0.1, params.sample_rate),
        "chirp"     => chirp(SIGNAL_LENGTH, 1.0, 50.0, params.sample_rate),
        _           => sine(SIGNAL_LENGTH, params.freq, params.sample_rate),
    }
}

fn main() {
    let params = parse_params();
    let signal = create_signal(&params);

    // Plot signal
    let idx: Vec<usize> = (0..signal.length()).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, signal.data, &[Color("red")]);
    fg.show();
}
