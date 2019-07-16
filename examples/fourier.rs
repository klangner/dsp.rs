use gnuplot::{Figure, Color};

use dsp::{SourceNode, ProcessingNode};
use dsp::generators::{SineGen, GenNode};
use dsp::fft::{ForwardFFTNode, InverseFFTNode};

static SAMPLE_FREQ: f32 = 256.0;
static FRAME_SIZE: usize = SAMPLE_FREQ as usize;


fn main() {
    
    // Nodes
    let mut gen = GenNode::new(Box::new(SineGen::new(4.0)), SAMPLE_FREQ, FRAME_SIZE);
    let mut fft = ForwardFFTNode::new(FRAME_SIZE);
    let mut ifft = InverseFFTNode::new(FRAME_SIZE);

    // Process graph
    let input = gen.next_frame();
    let spectrum = fft.process(input);
    let output_real = ifft.process(spectrum);

    // Visualize
    let idx: Vec<usize> = (0..FRAME_SIZE).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, input, &[Color("red")]);
    fg.axes2d().lines(&idx, output_real, &[Color("blue")]);
    fg.show();
}
