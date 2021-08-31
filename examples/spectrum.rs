#[macro_use]
extern crate clap;

use gnuplot::*;
use clap::{Arg, App};
use dsp::signal::Signal;
use dsp::generator::*;
use dsp::fft::*;


const SIGNAL_LENGTH: usize = 10*256;


// Application params
#[derive(Debug)]
struct Params {
    gen_name: String,
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
                .get_matches();
    let gen_name = args.value_of("gen").unwrap_or("sine"); 
    let freq = value_t!(args, "freq", f32).unwrap_or(4.0);
    Params { gen_name: gen_name.to_string(),
             freq: freq }
}

/// Create signal
fn create_signal(gen_name: &str, freq: f32, sample_rate:usize) -> Signal {
    match gen_name.as_ref() {
        "sawtooth"  => sawtooth(SIGNAL_LENGTH, freq, sample_rate),
        "square"    => square(SIGNAL_LENGTH, freq, sample_rate),
        "noise"     => noise(SIGNAL_LENGTH, 0.1, sample_rate),
        "chirp"     => chirp(SIGNAL_LENGTH, 1.0, 50.0, sample_rate),
        _           => sine(SIGNAL_LENGTH, freq, sample_rate),
    }
}


fn main() {
    let params = parse_params();
    let num_spectrums = 10;
    let window_size = SIGNAL_LENGTH / num_spectrums;
    let signal = create_signal(&params.gen_name, params.freq, window_size);
    let mut fft = ForwardFFT::new(window_size);

    // Split signal into frames
    let ps: Vec<f32> = (0..num_spectrums).flat_map(|i| {
        let (x1, x2) = (i*window_size, ((i+1)*window_size));
        let output: Vec<f32> = fft.process_real(&signal.data[x1..x2]);
        output.iter().take(window_size/2).map(|i| *i).collect::<Vec<f32>>()
    }).collect();

    plot_spectrogram(window_size/2, num_spectrums, &ps, window_size as f32/2.0);
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
	fg.show().unwrap();    
}
