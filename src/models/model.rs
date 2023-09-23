use  anyhow::Result;

use crate::bounded_float::{Amplitude32, Probability64};

#[derive(Debug)]
pub struct Model;

impl Model {
    pub fn new() -> Self {
        todo!()
    }

    pub fn process<'a>(&self, sample: Vec<Amplitude32>, phonemes: Option<&'a [String]>) -> Result<Vec<(&'a str, Probability64)>> {
        todo!()
    }
}