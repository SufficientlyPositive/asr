mod processing_api;
mod bound_float;
mod audio;

use crate::processing_api::PhonemeRetriever;
use crate::bound_float::Amplitude32;

const DEFAULT_SAMPLE_SIZE: usize = 10;
const DEFAULT_SAMPLE_RATE: i32 = 100;

fn main() {
    println!("Hello World!");
    
    let phonemes: Vec<&str> = vec!["m", "n", "ŋ", "p", "b", "t"];
    let a = processing_api::new_dummy(&phonemes, DEFAULT_SAMPLE_SIZE, DEFAULT_SAMPLE_RATE);

    // import samples here (likely generated using python script and saved somewhere for retestability)
    let sample1: Vec<bound_float::Amplitude32>  = Amplitude32::to_bounded_vec(vec![0.1, 0.56, 0.2, -0.31, -0.44, -0.2, -0.01, 0.3, -0.2, -0.4]);
    let sample2: Vec<bound_float::Amplitude32>  = Amplitude32::to_bounded_vec(vec![0.1, 0.56, 0.2, -0.31, -0.44, -0.2, -0.01, 0.3, -0.2, -0.4]);

    println!("{:?}", a.find_likelihood(sample1));
    println!("{:?}", a.find_sorted_likelihood(sample2));

}
