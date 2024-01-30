use std::fs::File;
use gnuplot::{Figure, Color, AxesCommon};
use dsp::node::ProcessNode;
use dsp::core::correlation::*;


const SAMPLE_RATE: usize = 100;
const WINDOW_SIZE: usize = 3*SAMPLE_RATE;


fn read_mat_file(fname: &str) -> Vec<f32> {
    let file = File::open(fname).expect("Wrong file");
    let mat_file = matfile::MatFile::parse(file).unwrap();
    let array = &mat_file.arrays()[0];
    let data: Vec<f32> = if let matfile::NumericData::Double { real, imag: _ } = array.data() {
        real.iter().map(|i| *i as f32).collect()
    } else {
        vec![]
    };
    data
}


fn main() {
    let buffer = read_mat_file("examples/assets/ppg.mat");
    let mut ac = AutoCorrelation::new(WINDOW_SIZE); 
    let mut corr_buffer = vec![0.; buffer.len() - WINDOW_SIZE];
    let _ = ac.process_buffer(&buffer, &mut corr_buffer);

    println!("Buffer length: {}", buffer.len());
    
    // Plot signal with ms as units
    let idx: Vec<usize> = (0..corr_buffer.len()).map(|i| i * 1000 / SAMPLE_RATE).collect();
    let mut fg = Figure::new();
    fg.set_title("Auto correlation");
    let axes = fg.axes2d();
    axes.lines(&idx, corr_buffer, &[Color("red")]);
    axes.set_x_label("Time in ms", &[]);
    fg.show().unwrap();
}