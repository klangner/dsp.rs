#[macro_use]
extern crate clap;

use gnuplot::*;
use clap::{Arg, App};
use dsp::num_complex::Complex32;
use dsp::node::*;
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
fn create_generator(gen_name: &str, freq: f32, sample_rate:usize) -> Box<dyn SourceNode<f32>> {
    match gen_name.as_ref() {
        "sawtooth"  => Box::new(Sawtooth::new(freq, sample_rate)),
        "square"    => Box::new(Square::new(freq, sample_rate)),
        "noise"     => Box::new(Noise::new(0.1)),
        "chirp"     => Box::new(Chirp::new(4.0, 1.0, 10.0, sample_rate)),
        _           => Box::new(Sinusoid::new(freq, sample_rate)),
    }
}


fn main() {
    let params = parse_params();
    let num_spectrums = 10;
    let window_size = SIGNAL_LENGTH / num_spectrums;
    let mut generator = create_generator(&params.gen_name, params.freq, window_size);
    let r2c = RealToComplex::new();
    let c2r = ComplexToReal::new();
    let fft = ForwardFFT::new(window_size);
    let mut buffer1 = vec![0.0; SIGNAL_LENGTH];
    let mut buffer2 = vec![Complex32::new(0., 0.); SIGNAL_LENGTH];
    let mut buffer3 = vec![Complex32::new(0., 0.); window_size];
    let mut buffer4 = vec![0.; window_size];
    
    generator.write_buffer(&mut buffer1);
    r2c.process_buffer(&buffer1, &mut buffer2);

    // Split signal into frames
    let ps: Vec<f32> = (0..num_spectrums).flat_map(|i| {
        let (x1, x2) = (i*window_size, ((i+1)*window_size));
        fft.process_buffer(&buffer2[x1..x2], &mut buffer3);
        c2r.process_buffer(&buffer3, &mut buffer4);
        buffer4[0..window_size/2].to_owned()
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
