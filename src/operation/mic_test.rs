use crate::{cli::args::CLArgs, models::{processing_api::PhonemeRetriever, model_frontend}};

use cpal::traits::{DeviceTrait, StreamTrait};
use anyhow::Result;

use super::mic_stream_util;

// for testing, default for now
// anything that goes into here is alpha variation of what is going into the final thing. All pre first refactor.
pub fn main(args: CLArgs) -> Result<()> {
    info!("{:?} operation is being performed. Audio will be processed for ~5 seconds through a dummy algorithm to check that the microphone is working as intended.", args.operation);

    let input_device = mic_stream_util::get_default_audio_input();
    let input_device_config: cpal::StreamConfig = input_device.default_input_config()?.into();
    debug!("Using input device: \"{}\" with default config \"{:?}\".", input_device.name()?, input_device_config);

    let model_sample_size = args.sample_size;
    let model_sample_rate = args.sample_rate; 
    debug!("Model is being fed inputs of {} audio samples at a rate of {} per second.", model_sample_size, model_sample_rate);

    let (input_fn, mut output_fn) = mic_stream_util::build_buffer_functions(model_sample_size, model_sample_rate, input_device_config.sample_rate.0, input_device_config.channels)?;

    let phonemes: Vec<&str> = vec!["m", "n", "ŋ", "p", "b", "t"];
    let dummy = model_frontend::new_dummy(&phonemes, model_sample_size as usize, &input_device_config);

    let input_stream = input_device.build_input_stream(&input_device_config, input_fn, mic_stream_util::err_fn, None)?;
    input_stream.play()?;

    // look at this? https://docs.rs/tokio/latest/tokio/time/fn.interval.html
    // alternatively look at the threadscope api https://www.reddit.com/r/learnrust/comments/v946ob/how_to_run_a_function_every_x_milliseconds/
    // https://ryhl.io/blog/actors-with-tokio/ this seems somewhat intriguing if not immediately relevant
    std::thread::scope(|s| {
        let (sender, receiver) = std::sync::mpsc::channel();

        s.spawn(move || {
            let start = std::time::Instant::now();
            let interval = std::time::Duration::from_secs_f32(1.0 / model_sample_rate as f32);
            let mut next_block = start + interval;

            while start.elapsed() < std::time::Duration::from_millis(5000) {

                // this doesn't seem to go fast enough in dev build to sample 50 times a second!
                match output_fn(model_sample_size as usize) {
                    Ok(sample) => sender.send(sample).unwrap(),
                    Err(a) => error!("{}", a),
                }                

                std::thread::sleep(next_block - std::time::Instant::now());
                next_block += interval;
                // if next_block < std::time::Instant::now() { error!("Sampling is taking too long to be at the given rate!"); return }
            }
        });

        let mut i = 0;

        // when actually using ML algo it might be important to implement proper thread-pool multithreading for using the ML model to retrieve the phonemes.
        for item in receiver {
            // e.g. for item in receiver send to threadpool to process wiht find_sorted_likelihood or similar.
            info!("{:?}", dummy.find_sorted_likelihood(item).unwrap());
            i += 1;
        }

        info!("Number of samples retrieved was: {i}. The expected number was {}. This gives {}% efficiency", 5 * model_sample_rate, i as f32 * 20.0 / model_sample_rate as f32);
    });

    // look at https://github.com/RustAudio/cpal/blob/master/examples/feedback.rs for how to create streams from here
    // note how they use HeapRb, from https://docs.rs/ringbuf/latest/ringbuf/ crate, can do the same just make consumer copy a certain amount of data
    // additionally need to figure out how many samples should be sent per second and how large they should be... (maybe 50-100/sec?)
    // ideally the first x elements should be pulled from whatever data struct is used, and next y should be copied. x = samplingRate / nLargeSamplesPerSecond, y = largeSampleSize - x.

    drop(input_stream);
    info!("Finished mic testing.");
    Ok(())
}