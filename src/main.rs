// REMOVE THIS WHEN SOMETHING CLOSE TO DECENT IS MADE
#![ allow( dead_code, unused_imports, unused_variables ) ]

mod processing_api;
mod bound_float;
mod bounded_float;
mod audio;

use crate::{processing_api::PhonemeRetriever, audio::SampleRate};
use crate::bound_float::Amplitude32;

use std::{path::Path, error::Error, sync::mpsc::Receiver};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb,Rb};

const DEFAULT_SAMPLE_SIZE: usize = 10;
const DEFAULT_SAMPLE_RATE: i32 = 100;

// for running as CLI
// this is terrible, if ever to make this more complicated use clap crate (as opposed to cpal, is confusing...)
fn main() {
    
    let mut args: Vec<String> = Vec::new();

    for arg in std::env::args() {
        args.push(arg);
    }

    match args.len() {
        1 => { match testing() {
            Ok(_) => {}
            Err(e) => {eprint!("{}", e)}
        }; 
        }
        3 => {
            let mode = &args[1];
            let phoneme_file_path: &Path = Path::new(&args[2]);

            if mode.eq("cont") || mode.eq("continuous") {
                continuous_processing(phoneme_file_path);
            } else {
                println!("Expected arguments when run with 2 args: [\"cont\"] [...phoneme_file_path...]");
            }
        }
        4 => {
            let mode = &args[1];
            let phoneme_file_path: &Path = Path::new(&args[2]);
            let audio_file_path: &Path = Path::new(&args[3]);

            if mode.eq("file") {
                process_file(phoneme_file_path, audio_file_path);
            } else {
                println!("Expected arguments when run with 3 args: [\"file\"] [...phoneme_file_path...] [...audio_file_path...]");
            }
        }
        _ => { println!("Incorrect number of arguments specified: 0, 2 or 3 args supported") }
    }

}

// for processing a file and spitting out words within it
fn process_file(phoneme_file_path: &Path, audio_file_path: &Path) {

}

// for processing a continous microphone stream
fn continuous_processing(phoneme_file_path: &Path) {

}

// for testing, default for now
// anything that goes into here is alpha variation of what is going into the final thing. All pre first refactor.
fn testing() -> anyhow::Result<()> {
    println!("In the default branch of funtionality. Used for testing purposes only.");
    
    // let phonemes: Vec<&str> = vec!["m", "n", "ŋ", "p", "b", "t"];
    // let a = processing_api::new_dummy(&phonemes, DEFAULT_SAMPLE_SIZE, DEFAULT_SAMPLE_RATE);

    // // import samples here (likely generated using python script and saved somewhere for retestability)
    // let sample1: Vec<bound_float::Amplitude32>  = Amplitude32::to_bounded_vec(vec![0.1, 0.56, 0.2, -0.31, -0.44, -0.2, -0.01, 0.3, -0.2, -0.4]);
    // let sample2: Vec<bound_float::Amplitude32>  = Amplitude32::to_bounded_vec(vec![0.1, 0.56, 0.2, -0.31, -0.44, -0.2, -0.01, 0.3, -0.2, -0.4]);

    // println!("{:?}", a.find_likelihood(sample1));
    // println!("{:?}", a.find_sorted_likelihood(sample2));

    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("failed to find input device");
    println!("Using input device: \"{}\"", input_device.name()?);

    // retrieve default config from input device
    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    // definitely should be messed with to see...
    // represent the samples being pushed to ml algo, sample rate is in samples/second
    // also this channeless s**t needs to go
    let channeless_input_sample_size: usize = 3000;
    let input_sample_size = channeless_input_sample_size * config.channels as usize;

    let input_sample_rate: usize = 50;

    let channeless_sample_throughput: usize = if config.sample_rate.0 % input_sample_rate as u32 == 0 {
        config.sample_rate.0 as usize / input_sample_rate
    } else {
        // also this is garbage, figure how to wrap this in an error instead of doing this...
        println!("Sample rate of input device ({}) should be divisible by block sample rate ({}) exactly.", config.sample_rate.0, input_sample_rate);
        return Ok(());
    };
    // the number of samples needing to be removed every time a large sample is sent through to the ml sample
    let sample_throughput = channeless_sample_throughput * config.channels as usize;
    println!("Sample throughput: {}", sample_throughput);


    let sample_save: usize = if input_sample_size > sample_throughput {
        input_sample_size - sample_throughput
    } else {
        // again this is garbage, see above
        println!("ML sample size ({}) and sample rate ({}) should be sufficient that sample throughput is less than ", config.sample_rate.0, input_sample_rate);
        return Ok(());
    };

    println!("Input device sample rate: {}", config.sample_rate.0);
    println!("With an ML sample size of {} and a rate of {} per second, buffer deletion sits at {} per sample, with {} equal between samples.", channeless_input_sample_size, input_sample_rate, channeless_sample_throughput, sample_save / config.channels as usize);
    println!("This leads to {}% matching material in each sample", (sample_save * 100) as f32 / input_sample_size as f32);

    let ring_buffer = HeapRb::<f32>::new(input_sample_size + 4 * sample_throughput);
    let (mut buffer_producer, mut buffer_consumer) = ring_buffer.split();

    // push 1 sample of zeroes onto the stack, this should allow for output to work instantaneously...
    for _ in 0..(input_sample_size + 2* sample_throughput) {
        buffer_producer.push(0.0).unwrap();
    }

    println!("Number of elements in buffer before starting throughput: {}", buffer_producer.len());

    let input_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        // potential optimisation? if in same thread just break after is_err()
        let mut output_fell_behind = false;
        for &sample in data {
            if buffer_producer.push(sample).is_err() {
                output_fell_behind = true;
            }
        }
        // println!("Data being pushed: {:?}", data);
        if output_fell_behind {
            eprintln!("Buffer consumer is not processing fast enough.")
        }
    };
    
    // data is assumed to be initialised to 0.0 already...
    let mut output_fn = move |data: &mut Vec<Amplitude32>| {
        let mut input_fell_behind = false;
        for i in 0..sample_throughput {
            data[i] = match buffer_consumer.pop() {
                Some(s) => Amplitude32::new(s),
                None => {
                    input_fell_behind = true;
                    Amplitude32::new(0.0)
                }
            };
        }
        
        // this is probably a shit way of doing this...
        // and doesn't seem to be stable, what if buffer doesn't have "max" elems to take?
        // TODO: requires reworking...
        let mut i: usize = 0;
        let max = data.len() - sample_throughput;
        for sample in buffer_consumer.iter().take(max) {
            data[i + sample_throughput] = Amplitude32::new(*sample);
            i += 1;
        }

        if input_fell_behind {
            // eprintln!("Buffer producer is not adding elements to buffer fast enough!");
        }
        if i != max {
            // println!("{} samples missing from buffer! Samples in buffer: {}.", (max - i) / config.channels as usize, buffer_consumer.len());
        }
        println!("{} samples missing from buffer! Samples in buffer: {}.", (max - i) / config.channels as usize, buffer_consumer.len() / config.channels as usize);
    };

    let phonemes: Vec<&str> = vec!["m", "n", "ŋ", "p", "b", "t"];
    let dummy = processing_api::new_dummy(&phonemes, input_sample_size, &config);

    println!("Attempting to build stream with f32 samples and `{:?}`.", config);
    let input_stream = input_device.build_input_stream(&config, input_fn, err_fn, None)?;
    input_stream.play()?;

    // create thread that utilises dummy.like2(v) as many times as it needs per second from the stream
    // look at this? https://docs.rs/tokio/latest/tokio/time/fn.interval.html
    // alternatively look at the threadscope api https://www.reddit.com/r/learnrust/comments/v946ob/how_to_run_a_function_every_x_milliseconds/
    // https://ryhl.io/blog/actors-with-tokio/ this seems somewhat intriguing if not immediately relevant

    // maybe give some latency here? Make the buffer larger?
    // aight this seems to sample way too fast xd
    std::thread::scope(|s| {
        let (sender, receiver) = std::sync::mpsc::channel();
        s.spawn(move || {
            let start = std::time::Instant::now();
            let interval = std::time::Duration::from_secs_f32(1.0 / input_sample_rate as f32);
            let mut next_block = start + interval;

            while start.elapsed() < std::time::Duration::from_millis(100) {
                let mut sample: Vec<Amplitude32> = vec![Amplitude32::new(0.0); input_sample_size];
                output_fn(&mut sample);

                sender.send(dummy.find_sorted_likelihood(sample)).unwrap();

                std::thread::sleep(next_block - std::time::Instant::now());
                next_block += interval;
            }
        });
        for item in receiver {
            println!("{:?}", item.unwrap());
        }
    });

    // look at https://github.com/RustAudio/cpal/blob/master/examples/feedback.rs for how to create streams from here
    // note how they use HeapRb, from https://docs.rs/ringbuf/latest/ringbuf/ crate, can do the same just make consumer copy a certain amount of data
    // additionally need to figure out how many samples should be sent per second and how large they should be... (maybe 50-100/sec?)
    // ideally the first x elements should be pulled from whatever data struct is used, and next y should be copied. x = samplingRate / nLargeSamplesPerSecond, y = largeSampleSize - x.

    drop(input_stream);
    println!("Done");
    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("An error occured on strea: {}", err);
}