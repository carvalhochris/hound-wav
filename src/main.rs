use hound;
use inquire::{InquireError, Select};
use std::any::type_name;
// use std::env;
use std::f32::consts::PI;
use std::i16;

fn main() {
    let options: Vec<&str> = vec!["Low", "Mid", "High"];
    let options2: Vec<&str> = vec!["2", "3", "4"];

    let ans: Result<&str, InquireError> = Select::new("Choose tone 1", options).prompt();

    let ans2: Result<&str, InquireError> = Select::new("Choose tone 2", options2).prompt();

    let ans2_uw = ans2.unwrap();

    let ans2_num: f32 = ans2_uw.parse().unwrap();

    // let ans2_type = type_name(ans2_uw);

    // let ans2_num = ans2_uw.parse().unwrap();

    println!("{}", ans2_uw);

    // println!("{}", ans2);
    // let answer = ans.unwrap();
    // println!("{}", answer);

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
                let sample = (t * freq * 0.5 * PI).sin();
                let sample2 = (t * freq2 * 0.5 * PI).sin();
                // let sum_sample = sample + sample2;
                let amplitude = i16::MAX as f32;
                let loudness = amplitude * 0.5;
                writer
                    .write_sample((sample * sample2 * loudness) as i16)
                    .unwrap();
            }
            // var_name
        }
        Err(_) => println!("There was an error, please try again"),
    }
}
