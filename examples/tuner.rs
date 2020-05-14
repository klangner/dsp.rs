
use std::env;
use dsp::{fft, spectrums};
use pitch_calc::calc::step_from_hz;
use pitch_calc::step::Step;


const SAMPLE_RATE: usize = 44100;
const FRAME_SIZE: usize = 4096;


fn main() {
    let file_path = env::args().nth(1).unwrap_or("examples/assets/sine_440hz.wav".to_string());
    let mut reader = audrey::open(file_path).unwrap();
    let samples: Vec<f32> = reader.samples().map(Result::unwrap).collect();
    let num_frames: usize = samples.len() / FRAME_SIZE;
    let mut fft = fft::ForwardFFT::new(FRAME_SIZE);

    (0..num_frames)
        .map(|i| &samples[(i*FRAME_SIZE)..((i+1)*FRAME_SIZE)])
        .map(|frame| fft.process_real(frame))
        .map(|output| spectrums::max_freq(&output, SAMPLE_RATE))
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