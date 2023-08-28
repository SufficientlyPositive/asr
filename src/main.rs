mod processing_api;
mod bound_float;
mod audio;

use crate::processing_api::PhonemeRetriever;
use crate::bound_float::Amplitude32;

use std::path::Path;

const DEFAULT_SAMPLE_SIZE: usize = 10;
const DEFAULT_SAMPLE_RATE: i32 = 100;

// for running as CLI
fn main() {
    // this is terrible, if ever to make this more complicated use clap crate
    let mut args: Vec<String> = Vec::new();

    for arg in std::env::args() {
        args.push(arg);
    }

    match args.len() {
        1 => { testing(); }
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
        _ => {}
    }

}

// for processing a file and spitting out words within it
fn process_file(phoneme_file_path: &Path, audio_file_path: &Path) {

}

// for processing a continous microphone stream
fn continuous_processing(phoneme_file_path: &Path) {

}

// for testing, default for now
fn testing() {
    println!("In the default branch of funtionality. Used for testing purposes only.");
    
    let phonemes: Vec<&str> = vec!["m", "n", "ŋ", "p", "b", "t"];
    let a = processing_api::new_dummy(&phonemes, DEFAULT_SAMPLE_SIZE, DEFAULT_SAMPLE_RATE);

    // import samples here (likely generated using python script and saved somewhere for retestability)
    let sample1: Vec<bound_float::Amplitude32>  = Amplitude32::to_bounded_vec(vec![0.1, 0.56, 0.2, -0.31, -0.44, -0.2, -0.01, 0.3, -0.2, -0.4]);
    let sample2: Vec<bound_float::Amplitude32>  = Amplitude32::to_bounded_vec(vec![0.1, 0.56, 0.2, -0.31, -0.44, -0.2, -0.01, 0.3, -0.2, -0.4]);

    println!("{:?}", a.find_likelihood(sample1));
    println!("{:?}", a.find_sorted_likelihood(sample2));
}
