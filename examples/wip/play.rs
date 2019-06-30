use cpal;

use dsp::generators::{SignalGen, SineGen};


fn main() {
    // Init device
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let buffer_size = format.sample_size();
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    // Init sine generator
    let mut gen = SineGen::new(4.0, format.sample_rate.0 as usize);
    let mut signal_buffer = vec![0.0; buffer_size];

    event_loop.run(move |_, data| {
        match data {
            // cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
            //     for sample in buffer.chunks_mut(format.channels as usize) {
            //         let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
            //         for out in sample.iter_mut() {
            //             *out = value;
            //         }
            //     }
            // },
            // cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
            //     for sample in buffer.chunks_mut(format.channels as usize) {
            //         let value = (next_value() * std::i16::MAX as f32) as i16;
            //         for out in sample.iter_mut() {
            //             *out = value;
            //         }
            //     }
            // },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                gen.next(&mut signal_buffer);
            },
            _ => (),
        }
    });}
