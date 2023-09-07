use std::iter::zip;

use super::{processing_api::PhonemeRetriever, model::Model};
use crate::bounded_float::{Amplitude32, Probability64};

// consider removing phonemes entirely? like what's the point??
pub struct ModelFrontend<'a> {
    phonemes: &'a Vec<&'a str>,
    model: Model
}

// if adding multithreading maybe make this async and await the result of process?
impl<'a> PhonemeRetriever<'a> for ModelFrontend<'a> {
    fn find_likelihood(&self, sample: Vec<Amplitude32>) -> Result<Vec<(&'a &'a str, Probability64)>, String> {
        self.model.process(sample)?
    }
}

impl<'a> ModelFrontend<'a> {
    pub fn new(phonemes: &'a Vec<&'a str>, model: Model) -> Self {
        ModelFrontend { phonemes, model }
    }
}

pub struct Dummy<'a> {
    phonemes: &'a Vec<&'a str>,
    sample_size: usize, 
    sample_meta: &'a cpal::StreamConfig
}

impl<'a> PhonemeRetriever<'a> for Dummy<'a> {
    fn find_likelihood(&self, sample: Vec<Amplitude32>) -> Result<Vec<(&'a &'a str, Probability64)>, String> {
        if sample.is_empty() { return Result::Err(String::from("Sample passed to find_likelihood is empty.")) }
        if sample.len() != self.sample_size { return Result::Err(format!("Sample passed to Dummy was expected to have size {}, but instead had size {}.", self.sample_size, sample.len())) }

        let mut likelihood: Vec<(&&str, Probability64)> = zip(self.phonemes, vec![0.0; self.phonemes.len()]).collect();

        let max = sample.iter().cloned().fold(f32::NEG_INFINITY, Amplitude32::max); // if so inclined to make sample not in range (-1, 1), divide by this instead
        let first_key = ((max + 1.0) * self.phonemes.len() as f32 / 2.00001).floor() as usize;
        let last_key = ((sample.last().unwrap() + 1.0) * self.phonemes.len() as f32 / 2.00001).floor() as usize;

        trace!("Max key: {} (from {}), Last key: {} (from {})", first_key, max, last_key, sample.last().unwrap());

        // assigning likelihood values
        if first_key == last_key {
            likelihood[first_key].1 = 1.0;
        } else {
            likelihood[first_key].1 = 0.6;
            likelihood[last_key].1 = 0.3;
        }

        Result::Ok(likelihood)
    }
}

// for testing purposes, generates a dummy process that returns based on the values of the first and last sample.
pub fn new_dummy<'a>(phonemes: &'a Vec<&'a str>, sample_size: usize, sample_meta: &'a cpal::StreamConfig) -> Dummy<'a> {
    Dummy { phonemes, sample_size, sample_meta }
}