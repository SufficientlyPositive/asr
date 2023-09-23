use crate::{cli::args::{CLArgs, Operation}, parsing, operation::mic_stream_util, multithreading::scoped_threadpool, models::{processing_api::PhonemeRetriever, model_frontend::new_dummy}};
use std::{sync::{mpsc, Arc, Mutex}, io};

use anyhow::Result;
use cpal::traits::{DeviceTrait, StreamTrait};

// for processing a continous microphone stream
// yeaaaaaaaaaaaaaaaaaaa this needs refactoring, particularly for more efficient thread usage cause fml this is not good.
pub fn main(args: CLArgs) -> Result<()> {
    info!("{:?} operation is being performed. Audio will be processed until this program is stopped.", args.operation);

    let input_device = mic_stream_util::get_default_audio_input();
    let input_device_config: cpal::StreamConfig = input_device.default_input_config()?.into();
    debug!("Using input device: \"{}\" with default config \"{:?}\".", input_device.name()?, input_device_config);

    let model_sample_size = args.sample_size;
    let model_sample_rate = args.sample_rate; 
    debug!("Model is being fed inputs of {} audio samples at a rate of {} per second.", model_sample_size, model_sample_rate);

    let (input_fn, mut output_fn) = mic_stream_util::build_buffer_functions(model_sample_size, model_sample_rate, input_device_config.sample_rate.0, input_device_config.channels)?;

    let phonemes: Vec<String> = parsing::parse_phoneme_file(args.phoneme_path)?;
    let mut model = parsing::parse_model_file(args.model_path)?;
    model.set_id_phonemes(phonemes.as_slice());

    let model_ref = &model;

    let input_stream = input_device.build_input_stream(&input_device_config, input_fn, mic_stream_util::err_fn, None)?;

    // return explicitly defined here for clarity
    // currently utilising 5 threads just for this...
    return std::thread::scope(|s| -> Result<()> {
        let (sender, retriever) = std::sync::mpsc::channel();
        let sender = Arc::new(Mutex::new(sender));
        let mut thread_pool = scoped_threadpool::Pool::new(3)?;

        s.spawn(move || {
            let interval = std::time::Duration::from_secs_f32(1.0 / model_sample_rate as f32);
            let mut next_block = std::time::Instant::now() + interval;

            // tbh could probably dip into unsafe here, since this is only being set in one thread and read in others.
            // alternatively look for the "Arc only" answer proposed below.
            let cancel_loop = Arc::new(Mutex::new(false));

            // it's probably better if futures are used here, that way we can always get a correct ordering of the futures
            // and we won't have to clone sender every fking time.
            thread_pool.scoped(|scope| {

                // wait for input key, stops program
                info!("Input any key to cease execution.");
                scope.execute(|| {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => info!("Stopping execution."),
                        Err(e) => warn!("{e}"),
                    }
                    *cancel_loop.lock().unwrap() = true;
                });

                // ideally this would not clone sender every time and sender would be inherent to some form of specialised scoped_threadpool with an output
                // this would require refactoring of scoped_threadpool, which will come eventually
                // also would be nice if there was a better way of cancelling that required an Arc only? Might have an answer: https://stackoverflow.com/questions/33911792/how-do-i-share-access-to-an-atomicbool-between-threads#33911915
                loop {
                    if *cancel_loop.lock().unwrap() { break; }
                    let sender = Arc::clone(&sender);
    
                    match output_fn(model_sample_size as usize) {
                        Ok(sample) => { scope.execute(move || {
                            sender.lock().unwrap().send(model_ref.find_sorted_likelihood(sample).unwrap()).unwrap();
                        })},
                        Err(a) => error!("{}", a),
                    }
    
                    std::thread::sleep(next_block - std::time::Instant::now());
                    next_block += interval;
                }
            });            
        });

        input_stream.play()?;

        for item in retriever {
            debug!("{:?}", item);
        }

        Ok(())
    });
}