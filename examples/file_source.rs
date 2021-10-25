use gnuplot::{Figure, Color, AxesCommon};
use dsp::runtime::node::SourceNode;
use dsp::node::file::FileSource;


const SAMPLE_RATE: usize = 512;


fn main() {
    let mut file_source = FileSource::new("target/example.data");
    let mut buffer = vec![0.0; SAMPLE_RATE];
    let _ = file_source.write_buffer(&mut buffer);

    // Plot signal with ms as units
    let idx: Vec<usize> = (0..buffer.len()).map(|i| i * 1000 / SAMPLE_RATE).collect();
    let mut fg = Figure::new();
    fg.set_title("Scope plot");
    let axes = fg.axes2d();
    axes.lines(&idx, buffer, &[Color("red")]);
    axes.set_x_label("Time in ms", &[]);
    fg.show().unwrap();
}