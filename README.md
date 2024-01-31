# Digital Signal Processing 

[![Rust](https://github.com/klangner/dsp.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/klangner/dsp.rs/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/dsp.svg)](https://crates.io/crates/dsp) [![Crates.io](https://img.shields.io/crates/l/dsp.svg)](https://github.com/klangner/dsp/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/dsp/badge.svg)](https://docs.rs/dsp/)

This library is focused in working with block of data not on real time processing.

If you are looking for a realtime processing library then there is already great library for it
https://github.com/FutureSDR/FutureSDR

The folder [examples](https://github.com/klangner/dsp/tree/master/examples) contains demo programs which shows how to use this library.


# Features
   
## Signal generators

Signals generators are nodes with the state. This allows to generate continuos signal across multiple buffers.
Implemented generators:
  
  * [x] Impulse
  * [x] Step
  * [x] Sinusoid
  * [x] Triangle
  * [x] Square
  * [x] Chirp
  * [x] Noise

## Filters

  * [x] Biquad
  * [x] Leaky Integrator (Exponential Smoothing)
  
## Window functions

  * [x] Rectangular
  * [x] Triangular
  * [x] Welch
  * [x] Sine
  * [x] Hann
  * [x] Hamming
  * [x] Blackman


## Frequency domain

  * [x] FFT forward and inverse using RustFFT crate.
  * [x] Find peak frequency


## Time domain

  * [x] Frequency shifter
  * [x] FM demodulation


# License

Licensed under [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
