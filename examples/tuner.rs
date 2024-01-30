use std::env;
use gnuplot::{Figure, Color, AxesCommon};
use dsp::node::*;
use dsp::core::{fft::*, complex::*, audio::audio_file::AudioFileSource};
use dsp::num_complex::Complex32;
use dsp::spectrum;
use pitch_calc::calc::step_from_hz;
use pitch_calc::step::Step;


const SAMPLE_RATE: usize = 44100;
const FRAME_SIZE: usize = 16385;
const MAX_FREQ: f32 = 1_000.;


fn main() {
    let file_path = env::args().nth(1).unwrap_or("examples/assets/sine_440hz.wav".to_string());

    let mut audio_src = AudioFileSource::new(&file_path);
    let mut r2c = RealToComplex::new();
    let mut fft = ForwardFFT::new(FRAME_SIZE, WindowType::Hamming);
    let mut buffer1 = vec![0.; FRAME_SIZE];
    let mut buffer2 = vec![Complex32::new(0., 0.); FRAME_SIZE];
    let mut buffer3 = vec![Complex32::new(0., 0.); FRAME_SIZE];


    for _ in 0..10 {
        let _ = audio_src.write_buffer(&mut buffer1);
        let _ = r2c.process_buffer(&buffer1, &mut buffer2);
        let _ = fft.process_buffer(&buffer2, &mut  buffer3);

        let freq = spectrum::max_freq(&buffer3, SAMPLE_RATE);
        if freq > 1.0 {
            let step = step_from_hz(freq);
            let letter = Step(step).letter_octave();
            println!("Freq: {}Hz, step: {}, letter: {:?}", freq, step, letter);
        } else {
            println!("Freq: {}Hz", freq);
        }
    }

    plot_freq(&buffer3);
}


fn plot_freq(buffer: &[Complex32]) {
    let res = SAMPLE_RATE as f32 / FRAME_SIZE as f32;
    let n = (MAX_FREQ / res) as usize;
    let idx: Vec< f32> = (0..n).map(|i| i as f32 * res).collect();
    let buf: Vec<f32> = buffer.iter().map(|e| e.norm()).collect();
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