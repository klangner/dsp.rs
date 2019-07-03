use gnuplot::{Figure, Color};
use num_complex::Complex32;
use dsp::{fft, spectrums};
use pitch_calc::calc::step_from_hz;
use pitch_calc::step::Step;


const SAMPLE_RATE: usize = 44100;
const SAMPLE_SIZE: usize = 4096;


fn main() {
    let mut ft = fft::ForwardFFT::new(SAMPLE_SIZE);
    let mut reader = audrey::open("examples/assets/sine_440hz.wav").unwrap();
    let mut samples: Vec<f32> = reader.samples().map(Result::unwrap).collect();

    samples.truncate(SAMPLE_SIZE);
    let mut input = samples.iter().map(|&x| Complex32::new(x, 0.0)).collect();
    let mut output = vec![Complex32::new(0.0, 0.0); SAMPLE_SIZE];
    ft.process(&mut input, &mut output);

    // Print estimated frequency
    let freq = spectrums::max_freq(&output, SAMPLE_RATE);
    let step = step_from_hz(freq);
    let letter = Step(step).letter_octave();
    println!("Freq: {}Hz, step: {}, letter: {:?}", freq, step, letter);

    // // Plot
    let powers: Vec<f32> = output.iter().map(|x| x.re).collect();
    let idx: Vec<f32> = (0..powers.len()/2).map(|x| (x as f32)* (SAMPLE_RATE as f32)/(SAMPLE_SIZE as f32)).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.show();
}
