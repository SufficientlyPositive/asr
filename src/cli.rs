use crate::{cli::args::{Operation, CLArgs}, operation};

use clap::Parser;

pub mod args;

pub fn cli_start() {
    let args = CLArgs::parse();

    match &args.operation {
        Operation::MicTest => match operation::start_mic_test(args) {
            Ok(_) => {}
            Err(e) => error!("{}", e)
        },
        Operation::Microphone => match operation::start_mic_input(args) {
            Ok(_) => {},
            Err(e) => error!("{}", e),
        },
        Operation::File { .. } => match operation::start_file_input(args) {
            Ok(_) => {},
            Err(e) => error!("{}", e),
        },
    }
}