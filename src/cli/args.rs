use std::path::PathBuf;

use clap::{Parser, Subcommand};

const DEFAULT_PHONEME_FILENAME: &'static str = "phonemes.txt";
const DEFAULT_MODEL_FILENAME: &'static str = "model.txt";
const DEFAULT_MODEL_FOLDER: &'static str = "model";


// ----------------------------------------------------------------------------------------


#[derive(Parser, Debug)]
#[command(name = "ASR in Rust", author, version, about, long_about = None)]
pub struct CLArgs {
    /// The number of samples to pass through to the model for phoneme identification
    #[arg(short = 's', long = "s_size", default_value_t = 3000)]
    pub sample_size: u16,

    /// The rate at which samples are passed through to the model for phoneme identification
    #[arg(short = 'r', long = "s_rate", default_value_t = 50)]
    pub sample_rate: u16,

    /// The path to the file in which to find phoneme categories for the model to use
    #[arg(short = 'p', long = "p_path", default_value = get_phoneme_file_path().into_os_string())]
    pub phoneme_path: PathBuf,

    /// The path to the file where the model is defined with its parameters
    #[arg(short = 'm', long = "m_path", default_value = get_model_file_path().into_os_string())]
    pub model_path: PathBuf,

    /// Which operation to perform using the model
    #[command(subcommand)]
    pub operation: Operation,
}

#[derive(Subcommand, Debug)]
pub enum Operation {
    /// Basic test to test whether audio setup functions correctly
    MicTest,

    /// Continuous operation, printing identified phones to console whenever they are picked up on the default microphone
    Microphone,

    /// Analyses and prints identified phones from an audio file
    File {
        /// File path to audio file
        file_path: PathBuf,
    },
}


// ----------------------------------------------------------------------------------------


fn get_current_working_directory() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path
}

fn get_phoneme_file_path() -> PathBuf {
    let mut path = get_current_working_directory();
    path.push(DEFAULT_MODEL_FOLDER);
    path.push(DEFAULT_PHONEME_FILENAME);
    path
}

fn get_model_file_path() -> PathBuf {
    let mut path = get_current_working_directory();
    path.push(DEFAULT_MODEL_FOLDER);
    path.push(DEFAULT_MODEL_FILENAME);
    path
}