#[macro_use]
extern crate clap;

use gnuplot::{Figure, Caption};
use clap::{Arg, App};
use num_complex::Complex32;
use dsp::ComplexBuffer;
use dsp::generators::*;
use dsp::fft::{ForwardFFT};


const SAMPLE_SIZE: usize = 2048;
const SIGNAL_LENGTH: f32 = 10.0;


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
                    .help("Sampling frequency")
                    .takes_value(true))
                .get_matches();
    let gen_name = args.value_of("gen").unwrap_or("chirp"); 
    let sample_rate = value_t!(args, "sample-rate", usize).unwrap_or(44100);
    let freq = value_t!(args, "freq", f32).unwrap_or(5_000.0);
    Params { gen_name: gen_name.to_string(),
             sample_rate: sample_rate,
             freq: freq }
}

/// Create Signal generator based on given params
fn create_generator(params: &Params) -> Box<SignalGen + 'static> {
    match params.gen_name.as_ref() {
        "triangle"  => Box::new(TriangleGen::new(params.freq, params.sample_rate)),
        "square"    => Box::new(SquareGen::new(params.freq, params.sample_rate)),
        "noise"     => Box::new(NoiseGen::new(0.4)),
        "chirp"     => Box::new(ChirpGen::new(5_000.0, 10_000.0, SIGNAL_LENGTH, params.sample_rate)),
        _           => Box::new(SineGen::new(params.freq, params.sample_rate)),
    }
}


fn main() {
    let params = parse_params();
    let mut gen = create_generator(&params);
    let mut ft = ForwardFFT::new(SAMPLE_SIZE);
    let mut spectrum: ComplexBuffer = vec![Complex32::new(0.0, 0.0); SAMPLE_SIZE];

    // Take as many spectrums as necessary to cover the whole signal length
    let num_spectrums = (SIGNAL_LENGTH * (params.sample_rate as f32) / (SAMPLE_SIZE as f32)) as usize;
    let ps: Vec<f32> = (0..num_spectrums).flat_map(|_| {
        let mut xs = (0..SAMPLE_SIZE).map(|_| Complex32::new(gen.next(), 0.0)).collect();
        ft.process(&mut xs, &mut spectrum);
        let out: Vec<f32> = spectrum[0..SAMPLE_SIZE/2].iter().map(|c| c.norm()).collect();
        out
    }).collect();

    plot_spectrogram(SAMPLE_SIZE / 2, num_spectrums, &ps, (params.sample_rate/2) as f32);
}

fn plot_spectrogram(height: usize, width: usize, data: &Vec<f32>, max_freq: f32) {
    let mut transposed = vec![0.0; height*width];
    transpose::transpose(&data, &mut transposed, height, width);

	let mut fg = Figure::new();
    fg.axes2d().image(
		transposed.iter(),
		height,
		width,
		Some((0.0, 0.0, SIGNAL_LENGTH as f64, max_freq as f64)),
		&[Caption("Frequency in Hz.")],
	);
	fg.show();    
}
