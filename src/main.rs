use hound;
use inquire::{InquireError, Select};
use std::f32::consts::PI;
use std::i16;

fn main() {
    let options: Vec<&str> = vec!["Low", "Mid", "High"];
    let options2: Vec<&str> = vec!["2", "3", "4", "5"];

    let ans: Result<&str, InquireError> = Select::new("Choose tone 1", options).prompt();

    let ans2: Result<&str, InquireError> = Select::new("Choose tone 2", options2).prompt();

    let ans2_uw = ans2.unwrap();

    let ans2_num: f32 = ans2_uw.parse().unwrap();

    println!("{}", ans2_uw);

    let mut freq: f32 = 500.0;

    let freq2: f32 = freq * ans2_num;

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
                freq = 200.0;
            }
            if the_choice == "High" {
                println!("making high tone");
                freq = 300.0;
            }

            let secs_num = 10;

            let num_samples = secs_num * 44100;

            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: 44100,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };

            let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

            for t in (0..num_samples).map(|x| x as f32 / 44100.0) {
                let sample = (t * freq * 0.5 * PI).sin();
                let sample2 = (t * freq2 * 0.5 * PI).sin();
                let amplitude = i16::MAX as f32;
                let loudness = amplitude * 0.5;
                writer
                    .write_sample((sample * sample2 * loudness) as i16)
                    .unwrap();
            }
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
