use crate::{cli::args::{CLArgs, Operation}, parsing, operation::mic_stream_util};

use anyhow::Result;
use cpal::traits::{DeviceTrait, StreamTrait};

// for processing a continous microphone stream
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
    let model = parsing::parse_model_file(args.model_path)?;

    let input_stream = input_device.build_input_stream(&input_device_config, input_fn, mic_stream_util::err_fn, None)?;
    input_stream.play()?;

    

    drop(input_stream);
    Ok(())
}