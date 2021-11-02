use std::fs::File;
use gnuplot::{Figure, Color, AxesCommon};


const SAMPLE_RATE: usize = 100;


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
    println!("Buffer length: {}", buffer.len());
    
    // Plot signal with ms as units
    let idx: Vec<usize> = (0..buffer.len()).map(|i| i * 1000 / SAMPLE_RATE).collect();
    let mut fg = Figure::new();
    fg.set_title("PPG");
    let axes = fg.axes2d();
    axes.lines(&idx, buffer, &[Color("red")]);
    axes.set_x_label("Time in ms", &[]);
    fg.show().unwrap();
}