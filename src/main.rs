use hound;
use std::env;
use std::f32::consts::PI;
use std::i16;

fn main() {
    
    // get cl args
    let args: Vec<String> = env::args().collect();
    // get arg 1
    let query: &String = &args[1];
    // get arg 2
    let seconds = &args[2];
    // convert arg 2 to int
    let secs_num = seconds.parse::<i32>().unwrap();
    // multiply arg 2 by sample rate
    let num_samples = secs_num * 44100;
    // convert freq arg to float
    let freq = query.parse::<f32>().unwrap();
    // init wav
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    // create file
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    // sample loop
    for t in (0..num_samples).map(|x| x as f32 / 44100.0) {
        let sample = (t * freq * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}
