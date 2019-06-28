use gnuplot::{Figure, Color};
use dsp::signals::{Signal};
use dsp::fft::{ForwardFFT};
use pitch_calc::calc::step_from_hz;
use pitch_calc::step::Step;

const SAMPLE_RATE: usize = 44100;
const SAMPLE_SIZE: usize = 4096;


fn main() {
    let mut ft = ForwardFFT::new(SAMPLE_SIZE);
    let mut reader = audrey::open("../assets/sine_440hz.wav").unwrap();
    let mut samples: Vec<f32> = reader.samples().map(Result::unwrap).collect();

    samples.truncate(SAMPLE_SIZE);
    let xs = Signal::from_reals(samples.iter().map(|&x| x as f64).collect(), SAMPLE_RATE);
    let spectrum = ft.process(&xs);

    // Print estimated frequency
    let freq = spectrum.max_freq() as f32;
    let step = step_from_hz(freq);
    let letter = Step(step).letter_octave();
    println!("Freq: {}Hz, step: {}, letter: {:?}", freq, step, letter);

    // Plot
    let powers: Vec<f64> = spectrum.to_vec().iter().map(|x| x.re).collect();
    let idx: Vec<f64> = (0..powers.len()/2).map(|x| (x as f64)* (SAMPLE_RATE as f64)/(SAMPLE_SIZE as f64)).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.show();
}
