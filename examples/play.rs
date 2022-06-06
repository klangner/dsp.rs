
use dsp::node::audio::AudioSink;

const SAMPLE_RATE: u32 = 44100;


fn main() {
    let _audio_sink = AudioSink::new(SAMPLE_RATE);
    std::thread::sleep(std::time::Duration::from_millis(1000));
}