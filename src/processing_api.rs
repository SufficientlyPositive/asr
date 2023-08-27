
use crate::bound_float::Amplitude32;
use crate::bound_float::Probability;
use crate::audio;

use core::result::Result;
use std::string::String;
use std::ops::Deref;
use std::iter::zip;

// TODO: refactor code to include audio sample and include methods for slicing larger audio recording into samples.
// Additionally, find a better way of propagating errors than using String of &'static str!!! Maybe here: https://www.reddit.com/r/rust/comments/r26itu/whats_the_best_way_of_reusing_error_messages/?
// Error handling: https://www.youtube.com/watch?v=g6WUHcyjsfc, this seems like a potential drain on performance...
// maybe instead of all that just write an error enum for potential issues?

// Lifetime translation
// - Lifetime a must be longer than Lifetime b. 
// - Lifetime b is the lifetime of the thing implementing PhonemeRetriever.
// - Lifetime a is the lifetime of the phoneme Vec.
//  (likely will also have to anotate lifetime in find_likelihood, as output should also have lifetime b (less than a))
pub trait PhonemeRetriever<'b> {
    fn new<'a: 'b>(phonemes: &'a Vec<&'a str>, sample_size: usize, sample_rate: i32) -> Self;

    fn find_likelihood(&self, sample: Vec<Amplitude32>) -> Result<Vec<(&&str, Probability)>, String>; // assumes all samples are normalised to range (-1,1), with 1 being max elem, may revisit this later
    
    fn find_sorted_likelihood(&self, sample: Vec<Amplitude32>) -> Result<Vec<(&&str, Probability)>, String> { // somewhat unsafe? assumes all f32 values != NaN (they should be in the range 0-1 anyways...).
        let mut l = self.find_likelihood(sample)?;
        l.sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        Ok(l)
    }
}

// for testing purposes, generates a dummy process that returns based on the values of the first and last sample.
pub fn new_dummy<'b, 'a: 'b>(phonemes: &'a Vec<&'a str>, sample_size: usize, sample_rate: i32) -> Dummy<'b> {
    Dummy::new(phonemes, sample_size, sample_rate)
}

pub struct Dummy<'a> {
    phonemes: &'a Vec<&'a str>,
    sample_size: usize, 
    sample_rate: i32
}

impl<'b> PhonemeRetriever<'b> for Dummy<'b> {

    fn new<'a: 'b>(phonemes: &'a Vec<&'a str>, sample_size: usize, sample_rate: i32) -> Dummy<'b> {
        if phonemes.is_empty() { panic!("Sending no phonemes to check on is kinda dumb. Also this needs a  more elegant way of returning.") }
        if sample_size < 1 { panic!("Attempting to instantiate dummy with less than 1 sample size is illegal.") }
        if sample_rate < 1 { panic!("Dummy should have a sample rate larger than 0!") }

        Dummy{ phonemes, sample_size, sample_rate }
    }

    fn find_likelihood(&self, sample: Vec<Amplitude32>) -> Result<Vec<(&&str, Probability)>, String> {
        if sample.is_empty() { return Result::Err(String::from("Sample passed to find_likelihood is empty.")) }
        if sample.len() != self.sample_size { return Result::Err(format!("Sample passed to Dummy was expected to have size {}, but instead had size {}.", self.sample_size, sample.len())) }

        let mut likelihood: Vec<(&&str, Probability)> = zip(self.phonemes, vec![Probability::new(0.0); self.phonemes.len()]).collect();

        // let max = sample.iter().cloned().fold(-1.0/0.0, f64::max); // if so inclined to make sample not in range (-1, 1), divide by this instead
        let first_key = ((sample.first().unwrap().as_f32() + 1.0) * self.phonemes.len() as f32 / 2.0).floor() as usize;
        let last_key = ((sample.last().unwrap().as_f32() + 1.0) * self.phonemes.len() as f32 / 2.0).floor() as usize;

        // assigning likelihood values
        if first_key == last_key {
            likelihood[first_key].1 = Probability::new(1.0);
        } else {
            likelihood[first_key].1 = Probability::new(0.6);
            likelihood[last_key].1 = Probability::new(0.3);
        }

        Result::Ok(likelihood)
    }
}
