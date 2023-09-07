
use crate::bounded_float::Amplitude32;
use crate::bounded_float::Probability64;

use core::result::Result;
use std::string::String;

// TODO: refactor code to include audio sample and include methods for slicing larger audio recording into samples.
// Additionally, find a better way of propagating errors than using String of &'static str!!! Maybe here: https://www.reddit.com/r/rust/comments/r26itu/whats_the_best_way_of_reusing_error_messages/?
// Error handling: https://www.youtube.com/watch?v=g6WUHcyjsfc, this seems like a potential drain on performance...
// maybe instead of all that just write an error enum for potential issues?

// Lifetime translation
// - Lifetime a must be longer than Lifetime b. 
// - Lifetime b is the lifetime of the thing implementing PhonemeRetriever.
// - Lifetime a is the lifetime of the phoneme Vec.
//  (likely will also have to anotate lifetime in find_likelihood, as output should also have lifetime b (less than a))
pub trait PhonemeRetriever<'a> {
    fn find_likelihood(&self, sample: Vec<Amplitude32>) -> Result<Vec<(&'a &'a str, Probability64)>, String>; // assumes all samples are normalised to range (-1,1), with 1 being max elem, may revisit this later
    
    fn find_sorted_likelihood(&self, sample: Vec<Amplitude32>) -> Result<Vec<(&'a &'a str, Probability64)>, String> { // somewhat unsafe? assumes all f32 values != NaN (they should be in the range 0-1 anyways...).
        let mut l = self.find_likelihood(sample)?;
        l.sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        Ok(l)
    }
}
