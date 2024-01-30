use gnuplot::{Figure, Color, AxesCommon};
use clap::Parser;
use dsp::num_complex::Complex32;
use dsp::core::{generator::Sine, fft::*, complex::*};


const FRAME_SIZE: usize = 8_192;
const SAMPLE_RATE: usize = 48_000;
const MAX_FREQ: f32 = 2_000.;


#[derive(Parser, Debug)]
struct Args {
    /// Center frequency
    #[clap(short, long, default_value_t = 100_000_000.0)]
    freq: f32,
}

fn main() {
    let args = Args::parse();
    let mut generator = Sine::new(args.freq, SAMPLE_RATE);
    let mut r2c = RealToComplex::new();
    let mut fft = ForwardFFT::new(FRAME_SIZE, WindowType::Hamming);
    let mut buffer1 = vec![0.0; FRAME_SIZE];
    let mut buffer2 = vec![Complex32::new(0., 0.); FRAME_SIZE];
    let mut buffer3 = vec![Complex32::new(0., 0.); FRAME_SIZE];

    let _ = generator.write_buffer(&mut buffer1);
    let _ = r2c.process_buffer(&buffer1, &mut buffer2);
    let _ = fft.process_buffer(&buffer2, &mut buffer3);

    // Plot signal with ms as units
    let res = SAMPLE_RATE as f32 / FRAME_SIZE as f32;
    let n = (MAX_FREQ / res) as usize;
    let idx: Vec< f32> = (0..n).map(|i| i as f32 * res).collect();
    let buf: Vec<f32> = buffer3.iter().map(|e| e.norm()).collect();
    let ref_point: f32 = buf.iter().sum();
    let data: Vec<f32> = buf[0..n].iter()
        .map(|e| 20. * f32::log10(2. * e / ref_point))
        .collect(); 
    let mut fg = Figure::new();
    fg.set_title("Frequency plot");
    let axes = fg.axes2d();
    axes.lines(&idx, &data, &[Color("red")]);
    axes.set_x_label("Freq (Hz)", &[]);
    axes.set_y_label("Power (dB)", &[]);
    fg.show().unwrap();
}