use gnuplot::{Figure, Color};
use num_complex::Complex32;

use dsp::{SourceNode, ProcessingNode, RealToComplexNode, ComplexToRealNode};
use dsp::generators::{SineGen, GenNode};
use dsp::fft::{ForwardFFT, InverseFFT};

static SAMPLE_FREQ: f32 = 256.0;
static DATA_LEN: usize = SAMPLE_FREQ as usize;


fn main() {
    
    // Nodes
    let mut gen = GenNode::new(Box::new(SineGen::new(4.0, SAMPLE_FREQ)));
    let mut rtc = RealToComplexNode::new();
    let mut ctr = ComplexToRealNode::new();
    let mut fft = ForwardFFT::new(DATA_LEN);
    let mut ifft = InverseFFT::new(DATA_LEN);

    // Buffers
    let mut input = vec![0.0; DATA_LEN];
    let mut input_complex = vec![Complex32::new(0.0, 0.0); DATA_LEN];
    let mut spectrum = vec![Complex32::new(0.0, 0.0); DATA_LEN];
    let mut output = vec![Complex32::new(0.0, 0.0); DATA_LEN];
    let mut output_real = vec![0.0; DATA_LEN];

    // Process graph
    gen.next_batch(&mut input);
    rtc.process(&input, &mut input_complex);
    fft.process(&mut input_complex, &mut spectrum);
    ifft.process(&mut spectrum, &mut output);
    ctr.process(&output, &mut output_real);

    // Visualize
    let idx: Vec<usize> = (0..DATA_LEN).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &input, &[Color("red")]);
    fg.axes2d().lines(&idx, &output_real, &[Color("blue")]);
    fg.show();
}
