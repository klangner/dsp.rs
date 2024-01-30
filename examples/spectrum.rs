use gnuplot::*;
use clap::Parser;
use dsp::num_complex::Complex32;
use dsp::runtime::node::*;
use dsp::node::{generator::*, fft::*, complex::*};


const SIGNAL_LENGTH: usize = 10*256;


#[derive(Parser, Debug)]
struct Args {
    /// Gain to apply to the seify source
    #[clap(short, long)]
    gen_name: String,

    /// Center frequency
    #[clap(short, long, default_value_t = 100_000_000.0)]
    freq: f32,
}

/// Parse command line arguments
/// Create signal
fn create_generator(gen_name: &str, freq: f32, sample_rate:usize) -> Box<dyn SourceNode<f32>> {
    match gen_name.as_ref() {
        "sawtooth"  => Box::new(Sawtooth::new(freq, sample_rate)),
        "square"    => Box::new(Square::new(freq, sample_rate)),
        "noise"     => Box::new(Noise::new(0.1)),
        "chirp"     => Box::new(Chirp::new(4.0, 1.0, 10.0, sample_rate)),
        _           => Box::new(Sine::new(freq, sample_rate)),
    }
}


fn main() {
    let args = Args::parse();
    let num_spectrums = 10;
    let window_size = SIGNAL_LENGTH / num_spectrums;
    let mut generator = create_generator(&args.gen_name, args.freq, window_size);
    let mut r2c = RealToComplex::new();
    let mut c2r = ComplexToReal::new();
    let mut fft = ForwardFFT::new(window_size, WindowType::Hamming);
    let mut buffer1 = vec![0.0; SIGNAL_LENGTH];
    let mut buffer2 = vec![Complex32::new(0., 0.); SIGNAL_LENGTH];
    let mut buffer3 = vec![Complex32::new(0., 0.); window_size];
    let mut buffer4 = vec![0.; window_size];
    
    let _ = generator.write_buffer(&mut buffer1);
    let _ = r2c.process_buffer(&buffer1, &mut buffer2);

    // Split signal into frames
    let ps: Vec<f32> = (0..num_spectrums).flat_map(|i| {
        let (x1, x2) = (i*window_size, ((i+1)*window_size));
        let _ = fft.process_buffer(&buffer2[x1..x2], &mut buffer3);
        let _ = c2r.process_buffer(&buffer3, &mut buffer4);
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
