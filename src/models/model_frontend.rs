use std::iter::zip;
use anyhow::{Result, anyhow};

use super::{processing_api::PhonemeRetriever, model::Model};
use crate::bounded_float::{Amplitude32, Probability64};

// consider removing phonemes entirely? like what's the point??
#[derive(Debug)]
pub struct ModelFrontend<'a> {
    phonemes: Option<&'a [String]>,
    model: Model
}

// if adding multithreading maybe make this async and await the result of process?
impl<'a> PhonemeRetriever<'a> for ModelFrontend<'a> {
    fn find_likelihood(&self, sample: Vec<Amplitude32>) -> Result<Vec<(&'a str, Probability64)>> {
        self.model.process(sample, self.phonemes)
    }
}

impl<'a> ModelFrontend<'a> {
    pub fn new(model: Model) -> Self {
        ModelFrontend { phonemes: None, model }
    }

    pub fn set_id_phonemes(&mut self, phonemes: &'a [String]) {
        self.phonemes = Some(phonemes);
    }
}

pub struct Dummy<'a> {
    phonemes: &'a [String],
    sample_size: usize, 
    sample_meta: &'a cpal::StreamConfig
}

impl<'a> PhonemeRetriever<'a> for Dummy<'a> {
    fn find_likelihood(&self, sample: Vec<Amplitude32>) -> Result<Vec<(&'a str, Probability64)>> {
        if sample.is_empty() { return Err(anyhow!("Sample passed to find_likelihood is empty.")) }
        if sample.len() != self.sample_size { return Err(anyhow!("Sample passed to Dummy was expected to have size {}, but instead had size {}.", self.sample_size, sample.len())) }

        let mut likelihood: Vec<(&'a str, Probability64)> = zip(self.phonemes.iter().map(String::as_str), vec![0.0; self.phonemes.len()]).collect();

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
pub fn new_dummy<'a>(phonemes: &'a [String], sample_size: usize, sample_meta: &'a cpal::StreamConfig) -> Dummy<'a> {
    Dummy { phonemes, sample_size, sample_meta }
}