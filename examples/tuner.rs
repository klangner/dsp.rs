use std::env;
use dsp::runtime::node::*;
use dsp::node::{fft::*, complex::*, audio::audio_file::AudioFileSource};
use dsp::num_complex::Complex32;
use dsp::spectrum;
use pitch_calc::calc::step_from_hz;
use pitch_calc::step::Step;


const SAMPLE_RATE: usize = 44100;
const FRAME_SIZE: usize = 4096;


fn main() {
    let file_path = env::args().nth(1).unwrap_or("examples/assets/sine_440hz.wav".to_string());

    let mut audio_src = AudioFileSource::new(&file_path);
    let r2c = RealToComplex::new();
    let fft = ForwardFFT::new(FRAME_SIZE);
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
}