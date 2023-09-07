use crate::cli::args::CLArgs;

use anyhow::Result;

mod mic_test;
mod mic_input;
mod file_input;
mod mic_stream_util;

pub fn start_mic_test(args: CLArgs) -> Result<()> { 
    mic_test::main(args)
}

pub fn start_mic_input(args: CLArgs) -> Result<()> {
    mic_input::main(args)
}

pub fn start_file_input(args: CLArgs) -> Result<()> {
    file_input::main(args)
}