use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, Device, InputCallbackInfo};
use anyhow::{Result, anyhow};
use ringbuf::HeapRb;

use crate::bounded_float::{Amplitude32, self};



pub fn get_default_audio_input() -> Device {
    let host = cpal::default_host();
    host.default_input_device().expect("No default audio device detected!")
}

// note: input only recieves data from 1 sample
pub fn build_buffer_functions(model_sample_size: u16, model_sample_rate: u16, mic_sample_rate: u32, mic_channels: u16) 
    -> Result<(
        impl FnMut(&[f32], &InputCallbackInfo) + Send, 
        impl FnMut(usize) -> Result<Vec<f32>> + Send)> {

            let (sample_throughput, sample_copy) = model_throughput_copy_split(model_sample_size, model_sample_rate, mic_sample_rate)?;
            debug!("Buffer throughput (samples deleted each input): {}. Buffer copy (samples remaining for proceeding inputs): {}.", sample_throughput, sample_copy);
            debug!("This leads to {}% matching samples across each input.", (sample_copy as u32 * 100) as f32 / model_sample_size as f32);
        
            let ring_buffer = HeapRb::<f32>::new((model_sample_size + ((model_sample_rate / 10) * sample_throughput)) as usize);
            let (mut buffer_producer, mut buffer_consumer) = ring_buffer.split();
        
            // push 1 sample of zeroes onto the stack, this should allow for output to work instantaneously...
            for _ in 0..(model_sample_size + 2 * sample_throughput) {
                buffer_producer.push(0.0).unwrap();
            }
        
            debug!("Number of elements in buffer before starting throughput: {}", buffer_producer.len());
        
            let input_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut output_fell_behind = false;

                let push_number = if data.len() % mic_channels as usize == 0 { data.len() } 
                    else { 
                        warn!("Input samples not fed complete data from all channels, skipping final sample (this may mess up future sample data as their channel alignment may be wrong).");
                        data.len() - data.len() % mic_channels as usize 
                    };

                for i in (0..push_number).step_by(mic_channels as usize) {
                    if buffer_producer.push(data[i]).is_err() {
                        output_fell_behind = true;
                    }
                }

                // println!("Data being pushed: {:?}", data);
                if output_fell_behind {
                    warn!("Buffer consumer is not processing fast enough.")
                }
            };
            
            let output_fn = move |sample_length: usize| -> Result<Vec<Amplitude32>> {
                if buffer_consumer.len() < sample_length {
                    return Err(anyhow!("Buffer producer is not adding elements to buffer fast enough!"))
                }
        
                let vec: Vec<Amplitude32> = bounded_float::cast_to_scaled_amplitude32(buffer_consumer.iter().take(sample_length).cloned().collect());
                buffer_consumer.skip(sample_throughput as usize);
        
                Ok(vec)
            };

    Ok((input_fn, output_fn))
}

pub fn err_fn(err: cpal::StreamError) {
    eprintln!("An error occured on stream: {}", err);
}

fn model_throughput_copy_split(model_sample_size: u16, model_sample_rate: u16, mic_sample_rate: u32) -> Result<(u16, u16)> {

    let model_throughput = 
        if mic_sample_rate % model_sample_rate as u32 == 0 { mic_sample_rate / model_sample_rate as u32 } 
        else { return Err(anyhow!("Sample rate of input device ({}) should be divisible by model input rate ({}) exactly.", mic_sample_rate, model_sample_rate)) };

    let model_copy = 
        if model_sample_size as u32 > model_throughput { model_sample_size - model_throughput as u16 }
        else { return Err(anyhow!("Model input rate ({}) and input size ({}) should be sufficient that the model throughput is less than or equal to a single input to the model.\n
                                    Consider input size to {} or altering input rate. Microphone sample rate: {}.", model_sample_rate, model_sample_size, model_throughput, mic_sample_rate)) };

    Ok((model_throughput as u16, model_copy))
}
