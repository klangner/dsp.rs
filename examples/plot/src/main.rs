use gnuplot::{Figure, Color};
use dsp::generators::*;


const N: usize = 1000;


fn main() {
    let xs = complex(3.0, 0.0).generate((0..N).map(|x| (x as f32)/(N as f32)).collect());
    let noise = noise(0.05).generate((0..N).map(|x| (x as f32)/(N as f32)).collect());
    let xs2 = xs.sum(&noise);

    let idx: Vec<usize> = (0..xs2.len()).collect();
    let ys: Vec<f32> = xs2.to_vec().iter().map(|x| x.re).collect();
    let mut fg = Figure::new();
    fg.axes2d().lines(&idx, &ys, &[Color("red")]);
    fg.show();
}