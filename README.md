# Digital Signal Processing 

[![Build Status](https://travis-ci.org/klangner/dsp.rs.svg?branch=master)](https://travis-ci.org/klangner/dsp.rs)
[![Crates.io](https://img.shields.io/crates/v/dsp.svg)](https://crates.io/crates/dsp) [![Crates.io](https://img.shields.io/crates/l/dsp.svg)](https://github.com/klangner/dsp/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/dsp/badge.svg)](https://docs.rs/dsp/)

**dsp is an early-stage open-source project**. It means that API can change at any time.
If you think that this library can help you, then let me know. We can discuss future direction and try to stabilize the API.

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


## Runtime
There is minimal support for runtime. 
Runtime is designed as a graph of nodes. 
Node is a single computation which takes buffer as an input and generate data 
to provided output buffer. It means that node do not allocate memory for data.
There are 3 types of node:
  * SourceNode - Generate new data
  * ProcessingNode - Processing data between buffers
  * SinkNode - Takes data from buffer and consumes it.
  
  
# License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
