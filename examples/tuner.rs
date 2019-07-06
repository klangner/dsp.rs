
use std::env;
use gnuplot::{Figure, Color};
use num_complex::Complex32;
use dsp::{RealBuffer, ComplexBuffer};
use dsp::{fft, spectrums};
use dsp::generators::SignalGen;
use pitch_calc::calc::step_from_hz;
use pitch_calc::step::Step;


const SAMPLE_RATE: usize = 44100;
const SAMPLE_SIZE: usize = 4096;


// Audio generator
pub struct AudioFileGen {
    samples: RealBuffer, 
    pos: usize
}

impl AudioFileGen {
    pub fn new(file_path: &str) -> AudioFileGen {
        let mut reader = audrey::open(file_path).unwrap();
        let samples = reader.samples().map(Result::unwrap).collect();
        AudioFileGen { samples, pos: 0 }
    }
}

impl SignalGen for AudioFileGen {

    fn next(&mut self) -> f32 {
        let sample = if self.pos < self.samples.len() { 
            self.samples[self.pos]
        } else {
            0.0
        };
        self.pos += 1;
        sample

    }

    fn has_next(&self) -> bool {
        self.pos < self.samples.len()
    }
}


fn main() {
    let file_path = env::args().nth(1).unwrap_or("examples/assets/sine_440hz.wav".to_string());
    let mut ft = fft::ForwardFFT::new(SAMPLE_SIZE);
    let mut gen = AudioFileGen::new(&file_path);
    let mut samples: Vec<Complex32> = vec![Complex32::new(0.0, 0.0); SAMPLE_SIZE];
    let mut output = vec![Complex32::new(0.0, 0.0); SAMPLE_SIZE];

    while gen.has_next() {
        for sample in samples.iter_mut() {
            *sample = Complex32::new(gen.next(), 0.0);
        }
        ft.process(&mut samples, &mut output);

        // Print estimated frequency
        let freq = spectrums::max_freq(&output, SAMPLE_RATE);
        if freq > 1.0 {
            let step = step_from_hz(freq);
            let letter = Step(step).letter_octave();
            println!("Freq: {}Hz, step: {}, letter: {:?}", freq, step, letter);
        } else {
            println!("Freq: {}Hz", freq);
        }
    }
}

pub fn plot_spectrum(spectrum: &ComplexBuffer) {
    let powers: Vec<f32> = spectrum.iter().map(|x| x.re).collect();
    let idx: Vec<f32> = (0..powers.len()/2).map(|x| (x as f32)* (SAMPLE_RATE as f32)/(SAMPLE_SIZE as f32)).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &powers, &[Color("blue")]);
    fg.show();
}