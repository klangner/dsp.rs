use std::env;
use dsp::node::*;
use dsp::fft::*;
use dsp::num_complex::Complex32;
use dsp::spectrum::*;
use pitch_calc::calc::step_from_hz;
use pitch_calc::step::Step;


const SAMPLE_RATE: usize = 44100;
const FRAME_SIZE: usize = 4096;


fn main() {
    let file_path = env::args().nth(1).unwrap_or("examples/assets/sine_440hz.wav".to_string());
    let mut reader = audrey::open(file_path).unwrap();
    let samples: Vec<f32> = reader.samples().map(Result::unwrap).collect();
    let num_frames: usize = samples.len() / FRAME_SIZE;
    let r2c = RealToComplex::new();
    let fft = ForwardFFT::new(FRAME_SIZE);
    let mut buffer2 = vec![Complex32::new(0., 0.); samples.len()];
    let mut buffer3 = vec![Complex32::new(0., 0.); FRAME_SIZE];

    r2c.process_buffer(&samples, &mut buffer2);

    (0..num_frames)
        .map(|i| {
            fft.process_buffer(&buffer2[(i*FRAME_SIZE)..((i+1)*FRAME_SIZE)], &mut  buffer3);
            let spectrum = Spectrum::new(buffer3.to_owned(), SAMPLE_RATE);
            spectrum.max_freq()
        })
        .for_each(|freq|  {
            if freq > 1.0 {
                let step = step_from_hz(freq);
                let letter = Step(step).letter_octave();
                println!("Freq: {}Hz, step: {}, letter: {:?}", freq, step, letter);
            } else {
                println!("Freq: {}Hz", freq);
            }
        });
}