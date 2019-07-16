
use std::env;
use gnuplot::{Figure, Caption};
use dsp::{RealBuffer, SourceNode, ProcessingNode};
use dsp::{fft, spectrums};
use pitch_calc::calc::step_from_hz;
use pitch_calc::step::Step;


const SAMPLE_RATE: usize = 44100;
const SAMPLE_SIZE: usize = 4096;
const REDUCE_FREQ: usize = 32;


// Audio generator
pub struct AudioFileGen {
    samples: RealBuffer, 
    pos: usize,
    output: RealBuffer,
}

impl AudioFileGen {
    pub fn new(file_path: &str, buffer_size: usize) -> AudioFileGen {
        let mut reader = audrey::open(file_path).unwrap();
        let samples = reader.samples().map(Result::unwrap).collect();
        let output = vec![0.0; buffer_size];
        AudioFileGen { samples, pos: 0, output }
    }

    fn has_next(&self) -> bool {
        self.pos < self.samples.len()
    }
}

impl SourceNode for AudioFileGen {
    type Buffer = RealBuffer;

    fn next_frame(&mut self) -> &RealBuffer {
        for sample in self.output.iter_mut() {
            *sample = if self.pos < self.samples.len() { 
                self.samples[self.pos]
            } else {
                0.0
            };
            self.pos += 1;
        }
        &self.output
    }
}


fn main() {
    let file_path = env::args().nth(1).unwrap_or("examples/assets/sine_440hz.wav".to_string());
    let mut gen = AudioFileGen::new(&file_path, SAMPLE_SIZE);
    let mut fft = fft::ForwardFFTNode::new(SAMPLE_SIZE);
    let mut spectrum: Vec<f32> = Vec::new();

    while gen.has_next() {
        let samples = gen.next_frame();
        let output = fft.process(samples);
        let out: Vec<f32> = output[0..SAMPLE_SIZE/REDUCE_FREQ].iter().map(|c| c.norm()).collect();
        spectrum.extend(out);

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

    plot_spectrogram(
        SAMPLE_SIZE/REDUCE_FREQ, 
        REDUCE_FREQ * spectrum.len() / SAMPLE_SIZE, 
        &spectrum, 
        SAMPLE_RATE as f32/REDUCE_FREQ as f32);
}

fn plot_spectrogram(height: usize, width: usize, data: &Vec<f32>, max_freq: f32) {
    let mut transposed = vec![0.0; height*width];
    transpose::transpose(&data, &mut transposed, height, width);
    let t = (width * SAMPLE_SIZE) as f64 / SAMPLE_RATE as f64;

	let mut fg = Figure::new();
    fg.axes2d().image(
		transposed.iter(),
		height,
		width,
		Some((0.0, 0.0, t, max_freq as f64)),
		&[Caption("Frequency in Hz.")],
	);
	fg.show();    
}