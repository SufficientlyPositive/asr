use std::path::PathBuf;
use anyhow::Result;

use crate::models::model_frontend::ModelFrontend;

mod model;
mod phonemes;

pub fn parse_phoneme_file(path: PathBuf) -> Result<Vec<String>> {
    todo!();
}

pub fn parse_model_file<'a>(path: PathBuf)  -> Result<ModelFrontend<'a>> {
    todo!()
}