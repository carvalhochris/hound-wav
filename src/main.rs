use hound;
use inquire::{InquireError, Select};
use std::env;
use std::f32::consts::PI;
use std::i16;

fn main() {
    let options: Vec<&str> = vec!["Low", "Mid", "High"];

    let ans: Result<&str, InquireError> =
        Select::new("What's your favorite fruit?", options).prompt();
    // let answer = ans.unwrap();
    // println!("{}", answer);

    let mut freq: f32 = 500.0;

    match ans {
        Ok(choice) => {
            let the_choice = choice;
            println!("You chose {}", the_choice);
            if the_choice == "Low" {
                println!("making low tone");
                freq = 100.0;
            }
            if the_choice == "Mid" {
                println!("making mid tone");
                freq = 500.0;
            }
            if the_choice == "High" {
                println!("making high tone");
                freq = 1000.0;
            }
            // get cl args
            // let args: Vec<String> = env::args().collect();
            // get arg 1
            // let query: &String = &args[1];
            // get arg 2
            // let seconds = &args[2];
            // convert arg 2 to int
            let secs_num = 10;
            // multiply arg 2 by sample rate
            let num_samples = secs_num * 44100;
            // convert freq arg to float
            // let freq = query.parse::<f32>().unwrap();
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
                let loudness = amplitude * 0.5;
                writer.write_sample((sample * loudness) as i16).unwrap();
            }
            // var_name
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
